use bincode::error;
use walkdir::{WalkDir, DirEntry};
use std::path::{Path};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::fs;
use rayon::prelude::*;
use crate::backup::{BackupMetadata, FileInfo, save_metadata, load_metadata};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::logs::{Logs};

pub fn exclude(entry: &DirEntry,exclude_file: &Vec<String>, exclude_dir: &Vec<String>) -> bool {
    
    let filename = match entry.file_name().to_str(){
        Some(name) => name, 
        None => return false,
    };

    let file_type = entry.file_type();

    if file_type.is_dir() {
        return exclude_dir.iter().any(|p| filename.contains(p));
    } 


    if file_type.is_file() {
        return exclude_file.iter().any(|p| filename.contains(p));
    } 


    false
}  


pub fn verify_file_backup(file_path: &str, checksum_expe: &str) -> bool{
    let d = checksum(file_path);
    let hash = match d {
        Ok(h) => h,
        Err(e) => {
            println!("Error: {}", e);
            String::new()
        }
    };


    if checksum_expe == hash {
        return true;
    }

    false
}

pub fn checksum(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {


    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();

    let mut buffer = [0; 4096];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let cal_hash = format!("{:x}", hasher.finalize());

    Ok(cal_hash)

}

pub fn get_current_timestamp() -> u64 {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH)
        .expect("Error data too old");
    
    duration.as_secs()  
}

pub fn get_last_modified(path: &str) -> u64 {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Errore metadata: {}", e);
            return 0;
        }
    };
    
    let modified = match metadata.modified() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Errore modified: {}", e);
            return 0;
        }
    };
    
    let duration = modified.duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    
    duration.as_secs()
}

pub fn make_backup(src: &str, dst: &str, exclude_file: &Vec<String>, exclude_dir: &Vec<String>) -> io::Result<()> {
    let _ = Logs::info("Normal backup");

    let source = Path::new(src);
    let dest = Path::new(dst);
    let mut metadata_bincode = BackupMetadata{files: Vec::new()};


    if !source.exists() || !source.is_dir() {
        let _ = Logs::error("Source directory not found");
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source directory not found"));
    }


    let walker = WalkDir::new(source).into_iter();

    let entries: Vec<_> = walker
        .filter_entry(|e| !exclude(e, &exclude_file, &exclude_dir))
        .filter_map(|e| e.ok())
        .collect();

    let results: Vec<_> = entries.par_iter().map(|file| -> io::Result<Option<FileInfo>> {
        let path = file.path();
        let relative_path = path.strip_prefix(source).
        map_err(|e| {
            let _ = Logs::error(&format!("Errore durante il backup: {}", e));
            io::Error::new(io::ErrorKind::InvalidData, e.to_string())})?;
        let dest_path = dest.join(relative_path);

        if file.file_type().is_dir() {
            std::fs::create_dir_all(&dest_path)?;
            Ok(None)
        } else {
            let prepath: &str = path.to_str().ok_or_else(|| {
                let _ = Logs::error("Invalid UTF-8 path");
                io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 path")
            })?;


             let hash = checksum(prepath).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                let _ = Logs::error(&format!("Error: {}", e));
                String::new()
            });
            
            fs::copy(&path, &dest_path)?;

            let predest: &str = dest_path.to_str().ok_or_else(|| {
                let _ = Logs::error("Invalid UTF-8 dest path");
                io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 dest path")
            })?;
            
            if !verify_file_backup(predest, &hash) {
                eprintln!("Backup verification failed for {}", predest);
                let _ = Logs::error(&format!("Backup verification failed for {}", predest));
            }
           let timestamp_modified = get_last_modified(prepath);
           let timestamp_backup = get_current_timestamp();
            


           Ok(Some(FileInfo {
                relative_path: relative_path.to_string_lossy().to_string(),
                hash: hash,
                timestamp_modified,
                timestamp_backup,
                deleted: false
            }))
        }
    
    }).collect();


    let mut success_count = 0;
    let mut error_count = 0;
    for result in results {
        match result {
            Ok(Some(file_info)) =>{ 
                success_count+= 1; 
                metadata_bincode.files.push(file_info)
            },
            Ok(None) => {},  
            Err(e) => {
                error_count+= 1; 
                let _ = Logs::error(&format!("Errore durante il backup: {}", e));
                eprintln!("Errore durante il backup: {}", e)
            },
        }
    }

    let _ = Logs::info(&format!("Backup completed {} file successed; {} errors", success_count, error_count));

    let _ = save_metadata(&metadata_bincode);
    let _ = Logs::info("Metadata bincode update");
    Ok(())
}

pub fn sync_backup(src: &str, dst: &str, exclude_file: &Vec<String>, exclude_dir: &Vec<String>) -> io::Result<()> {
    let _ = Logs::info("Sync backup");

    let source = Path::new(src);
    let dest = Path::new(dst);
    let mut metadata_bincode = BackupMetadata { files: Vec::new() };
    

    let existing_metadata = load_metadata().ok().map(|m| m.files).unwrap_or_default();

    if !source.exists() || !source.is_dir() {
        let _ = Logs::error("Source directory not found");
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source directory not found"));
    }

    let walker = WalkDir::new(source).into_iter();
    let entries: Vec<_> = walker
        .filter_entry(|e| !exclude(e, &exclude_file, &exclude_dir))
        .filter_map(|e| e.ok())
        .collect();

  
    let mut current_files = std::collections::HashSet::new();

    let results: Vec<_> = entries.par_iter().map(|file| -> io::Result<Option<FileInfo>> {
        let path = file.path();
        let relative_path = path.strip_prefix(source)
            .map_err(|e| {
                let _ = Logs::error(&format!("Errore durante il backup: {}", e));
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            })?;
        let dest_path = dest.join(relative_path);

        if file.file_type().is_dir() {
            std::fs::create_dir_all(&dest_path)?;
            return Ok(None);  
        }

        let prepath: &str = path.to_str()
            .ok_or_else(|| 
                {let _ = Logs::error("Invalid UTF-8 path");
                io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 path")}
            )?;
        
        let relative_path_str = relative_path.to_string_lossy().to_string();
        
        let existing_file = existing_metadata.iter()
            .find(|f| f.relative_path == relative_path_str);

        let should_backup = match existing_file {
            Some(file_info) => {
                let timestamp_modified = get_last_modified(prepath);
                timestamp_modified > file_info.timestamp_modified
            },
            None => true,  
        };

        if should_backup {
            let hash = checksum(prepath).unwrap_or_else(|e| {
                eprintln!("Error calculating checksum: {}", e);
                let _ = Logs::error(&format!("Error calculating checksum: {}", e));
                String::new()
            });

            fs::copy(&path, &dest_path)?;

            let predest: &str = dest_path.to_str()
                .ok_or_else(||{ 
                    let _ = Logs::error("Invalid UTF-8 dest path");
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 dest path")})?;
            
            if !verify_file_backup(predest, &hash) {
                eprintln!("Backup verification failed for {}", predest);
                let _ = Logs::error(&format!("Backup verification failed for {}", predest));
            }

            let timestamp_modified = get_last_modified(prepath);
            let timestamp_backup = get_current_timestamp();

            
            Ok(Some(FileInfo {
                relative_path: relative_path_str,
                hash,
                timestamp_modified,
                timestamp_backup,
                deleted: false,  
            }))
        } else {
            Ok(existing_file.map(|f| FileInfo {
                relative_path: f.relative_path.clone(),
                hash: f.hash.clone(),
                timestamp_modified: f.timestamp_modified,
                timestamp_backup: f.timestamp_backup,
                deleted: false, 
            }))
        }
    }).collect();

    let mut success_count = 0;
    let mut error_count = 0;
    for result in results {
        match result {
            Ok(Some(file_info)) => {
                current_files.insert(file_info.relative_path.clone());
                metadata_bincode.files.push(file_info);
                success_count += 1;
            },
            Ok(None) => {},
            Err(e) => {
                error_count += 1;
                let _ = Logs::error(&format!("Errore durante il backup: {}", e));
                eprintln!("Errore durante il backup: {}", e)
            },
        }
    }

    let _ = Logs::info(&format!("Backup completed {} file successed; {} errors", success_count, error_count));


    for old_file in existing_metadata.iter() {
        if !current_files.contains(&old_file.relative_path) && !old_file.deleted {
            metadata_bincode.files.push(FileInfo {
                relative_path: old_file.relative_path.clone(),
                hash: old_file.hash.clone(),
                timestamp_modified: old_file.timestamp_modified,
                timestamp_backup: get_current_timestamp(),
                deleted: true,
            });
        } else if old_file.deleted {
            metadata_bincode.files.push(old_file.clone());
            let _ = Logs::trace(&format!("Backup file deleted form source: {}", old_file.relative_path));
        }
    }

    let _ = save_metadata(&metadata_bincode);
    let _ = Logs::info("Metadata bincode update");
    Ok(())
}
