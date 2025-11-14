use bollard::Docker;


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