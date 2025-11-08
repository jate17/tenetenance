/*

    MOD

*/

mod system_monitor;
mod utils;
mod log;
mod backup;

/*

    USE

    use system_monitor::{cpu_check, ram_check, storage_check, get_users, get_temperature, process_info};
    use log::{clean_logs};
    use backup::make_backup;
*/
use backup::{make_backup, checksum, verify_file_backup, get_current_timestamp, get_last_modified, exclude};


use walkdir::{WalkDir, DirEntry};
use core::time;
use std::path::{Path};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::fs;
use rayon::prelude::*;
use crate::backup::{BackupMetadata, FileInfo, load_metadata, save_metadata};
use std::time::{SystemTime, UNIX_EPOCH};



fn main() -> std::io::Result<()>  {

    
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