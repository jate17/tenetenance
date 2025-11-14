/*

    MOD

*/

mod system_monitor;
mod utils;
mod clean;
mod backup;
mod logs;
mod json;
/*

    USE

    use system_monitor::{cpu_check, ram_check, storage_check, get_users, get_temperature, process_info};
    use clean::{clean_logs};
    use backup::{make_backup, sync_backup};
    
    use logs::{Logs};
    use backup::{sync_backup};
    use json::{transform_in_json};



*/










#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    Ok(())
}























/* 
    let mut exclude_file: Vec<String> = Vec::new();
    let mut exclude_dir: Vec<String>= Vec::new();
    exclude_dir.push(".git".to_string());
    exclude_file.push("bla".to_string());
    let _ = sync_backup("./log_test", "./backup", &exclude_file, &exclude_dir);
    */


/* let g = load_metadata();

    for file in &g.unwrap().files{
        println!("{:?}", file.relative_path);
    } 
 */


/*    
    let mut sys = System::new_all();

    sys.refresh_all();


    let users = Users::new_with_refreshed_list();
    for user in users.list() {
        println!("{:?}", user.name());
    }
 */