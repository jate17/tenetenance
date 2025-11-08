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
*/

use backup::make_backup;






fn main() -> Result<(), Box<dyn std::error::Error>> {

   
    let mut exclude_file: Vec<String> = Vec::new();
    let mut exclude_dir: Vec<String> = Vec::new();
    exclude_dir.push("target".to_string());
    exclude_dir.push(".git".to_string());
    exclude_file.push(".txt".to_string());

    make_backup("./log_test", "./test" , &exclude_file, &exclude_dir);
    Ok(())
}



fn sync_backup() {
     
    /*
        Struct data

            relative_path,
            hash,
            timestemp,
            backup_timestamp
            
    
     */



}



/*    
    let mut sys = System::new_all();

    sys.refresh_all();


    let users = Users::new_with_refreshed_list();
    for user in users.list() {
        println!("{:?}", user.name());
    }
 */