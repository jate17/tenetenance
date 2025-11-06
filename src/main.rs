/*

    MOD

*/

mod system_monitor;
mod utils;
mod log;



use std::{fs, time::{SystemTime, UNIX_EPOCH}};

/*

    USE

*/
use system_monitor::{cpu_check, ram_check, storage_check, get_users, get_temperature, process_info};
use log::{clean_logs};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
}







/*    
    let mut sys = System::new_all();

    sys.refresh_all();


    let users = Users::new_with_refreshed_list();
    for user in users.list() {
        println!("{:?}", user.name());
    }
 */