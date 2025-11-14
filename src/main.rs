/*

    MOD

*/

mod system_monitor;
mod utils;
mod clean;
mod backup;
mod logs;
/*

    USE

    use system_monitor::{cpu_check, ram_check, storage_check, get_users, get_temperature, process_info};
    use log::{clean_logs};
    use backup::{make_backup, sync_backup};


*/



use logs::{Logs};
use backup::{sync_backup};



use bollard::{Docker};
use bollard::query_parameters::ListContainersOptions;
use std::{collections::HashMap, io};


pub async fn connect_docker() -> Result<Docker, bollard::errors::Error> {
    // Connessione automatica (socket Unix o Named Pipe Windows)
    Docker::connect_with_local_defaults()
}



/*



id	Identificazione univoca
names	Identificazione nominale
image, image_id	Riferimento all’immagine
command	Comando di avvio
created	Data di creazione per politiche manutenzione
ports	Monitoraggio rete e porte
size_rw, size_root_fs	Uso disco e gestione spazio
labels	Categorizzazione e filtri
state, status	Monitoraggio stato e salute
host_config	Configurazione risorse host
network_settings	Configurazione rete e diagnostica
mounts	Gestione storage e volumi
*/
pub async fn get_docker_version() -> std::io::Result<()> {
    let docker = connect_docker().await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let opts = ListContainersOptions {
        all: true,
        limit: None,
        size: false,
        filters: Default::default(),
    };

    let containers_result = docker.list_containers(Some(opts)).await;

    match containers_result {
        Ok(containers) => {
           // println!("All containers: {:?}", containers);
            let d = containers[0].clone();

            println!("{:?}",d.id);

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to list containers: {:?}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}




#[tokio::main]
async fn main() -> std::io::Result<()> {
    get_docker_version().await?;  // Aspetta il risultato
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