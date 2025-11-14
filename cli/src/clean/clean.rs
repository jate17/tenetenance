use std::fs; 
use std::time::{SystemTime, UNIX_EPOCH};
use crate::logs::{Logs};


pub fn clean_logs(log_dir: &str, keep_days: u64) -> Result<usize, Box<dyn std::error::Error>> {
    let cutoff_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() - (keep_days * 86400);

    let mut deleted_count = 0;

    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();

        if  filename.ends_with(".log") || filename.ends_with(".txt") {
            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?
                .duration_since(UNIX_EPOCH)?
                .as_secs();

            if modified < cutoff_time {
                fs::remove_file(&path)?;
                deleted_count += 1;
            }
        }
    }
    let _ = Logs::info(&format!("Logs deleted: {}", deleted_count));
    Ok(deleted_count)
}


pub fn clean_temp_files(max_age_days: u64) -> Result<usize, Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir(); 
    let cutoff_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() - (max_age_days * 86400);

    let mut deleted_count = 0;

    for entry in fs::read_dir(&temp_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?
                .duration_since(UNIX_EPOCH)?
                .as_secs();

            if modified < cutoff_time {
                match fs::remove_file(&path) {
                    Ok(_) => {
                        deleted_count += 1;
                    }
                    Err(e) => {
                        let _ = Logs::error(&format!("Error temp files: {}", e));                    
                        println!("Errore: {}", e)
                    },
                }
            }
        }
    }
    let _ = Logs::info(&format!("Temp files deleted: {}", deleted_count));
    Ok(deleted_count)
}
