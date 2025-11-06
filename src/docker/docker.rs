use std::process::Command;

/*

Docker Command


- docker stats <container_id_o_nome>
- docker container stats
- docker ps
- docker ps -all
- docker logs -f <container_name>
- docker inspect <container_name> (or <container_id>)


*/

pub fn get_all_docker_stats()  -> Result<Vec<Ports>, Box<dyn std::error::Error>> {
    let output = Command::new("lsof")
        .arg("-i")
        .arg("-P")
        .arg("-n")  
        .output()?;
    
    let result = String::from_utf8_lossy(&output.stdout);
    

}