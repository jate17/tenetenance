/*

    MOD

*/

mod system_monitor;
mod utils;
mod log;


/*

    USE

    use system_monitor::{cpu_check, ram_check, storage_check, get_users, get_temperature, process_info};
use log::{clean_logs};
*/


use fs_extra::copy_items;
use walkdir::{WalkDir, DirEntry};

use std::fmt::format;
use std::path::{Path, PathBuf};
use std::task::RawWakerVTable;
use fs_extra::file::{copy_with_progress, CopyOptions, TransitProcess};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, BufReader, Read};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    /* 
    let mut exclude_file: Vec<String> = Vec::new();
    let mut exclude_dir: Vec<String> = Vec::new();
    exclude_dir.push("target".to_string());
    exclude_dir.push(".git".to_string());
    exclude_file.push(".txt".to_string());
    let walker = WalkDir::new("./").into_iter();
    for entry in walker.filter_entry(|e| !exclude(e, &exclude_file, &exclude_dir)){
        let entry = entry?;
        println!("{}", entry.path().display());
    }
    */
    let file_path = "./log_test/log_nginx.log";
    let d = checksum(file_path);
     let d = checksum(file_path);
    let hash = match d {
        Ok(h) => h,
        Err(e) => {
            println!("Error: {}", e);
            String::new()
        }
        
    };
    let s = verify_file_backup(file_path, hash);
    println!("{}", s);
    
    Ok(())
}


fn exclude(entry: &DirEntry,exclude_file: &Vec<String>, exclude_dir: &Vec<String>) -> bool {
    
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


fn verify_file_backup(file_path: &str, checksum_expe: String) -> bool{
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

fn checksum(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {


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


fn make_backup(src: &str, dst: &str, exclude_file: &Vec<String>, exclude_dir: &Vec<String>) -> io::Result<()> {


    let source = Path::new(src);
    let dest = Path::new(dst);

    if !source.exists() || !source.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source directory not found"));
    }

    let walker = WalkDir::new(source).into_iter();
    for entry in walker.filter_entry(|e| !exclude(e, &exclude_file, &exclude_dir)){
        let entry = entry?; 
        let path = entry.path();

        let relative_path = path.strip_prefix(source).unwrap();

        let dest_path = dest.join(relative_path);

        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&dest_path)?;
        }else{
            let mut opt = CopyOptions::new();
            opt.overwrite = true;
            
            copy_items(&[path], &dest_path.parent().unwrap(), &opt);
        }


    
    }
    Ok(())
}






/*    
    let mut sys = System::new_all();

    sys.refresh_all();


    let users = Users::new_with_refreshed_list();
    for user in users.list() {
        println!("{:?}", user.name());
    }
 */