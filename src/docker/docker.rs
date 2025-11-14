use bollard::Docker;
use bollard::query_parameters::ListContainersOptions;
use std::{collections::HashMap, io};


pub struct ManageDocker {
    Client: Docker
}


impl ManageDocker {

    pub async fn new() -> Result<Self, bollard::errors::Error>{
        let client = Docker::connect_with_local_defaults()?;
        Ok( ManageDocker {client})
    }

    pub async fn health(&self) -> bool {
        self.client.ping().await.is_ok()
    }

    pub async fn get_info(&self) -> Result<String, bollard::errors::Error> {
        let info = self.client.info().await?;

        

    } 



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
async fn get_docker_version() -> io::Result<()> {
    let docker = connect_docker().await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let opts = ListContainersOptions {
        all: true,
        limit: None,
        size: false,
        filters: Default::default(),
    };

    let containers_result = docker.list_containers(Some(opts)).await;

    match containers_result {
        Ok(containers) => {
            let json_string = transform_in_json("containers",&containers).unwrap();
            println!("{}", json_string);

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to list containers: {:?}", e);
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}


pub async fn connect_docker() -> Result<Docker, bollard::errors::Error> {
    // Connessione automatica (socket Unix o Named Pipe Windows)
    Docker::connect_with_local_defaults()
}
