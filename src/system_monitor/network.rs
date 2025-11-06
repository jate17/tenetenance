use sysinfo::{self, Networks};
use crate::utils::{conv_bytes_to_mb};
use std::process::Command;


/// Struct for Network interface,download, upload 
#[derive(Debug, Clone)]
pub struct Net {
    interface: String, 
    download: f64,
    upload: f64
}

///
/// Get network info 
/// 
/// # Return 
/// 
/// - Net struct inside the vector
/// 
pub fn get_network() -> Vec<Net> {
    let mut output: Vec<Net> = Vec::new();

    let mut networks = Networks::new_with_refreshed_list();
    
    networks.refresh(true);
    
    for (interface_name, data) in &networks {
        let d = conv_bytes_to_mb(data.total_received()); 
        let u = conv_bytes_to_mb(data.total_transmitted());
        
        output.push(
            Net {
                interface: interface_name.to_string(),
                download: d,
                upload: u
            }
        );
    }

    output    
}



/*
        0 = nome processo 
        1 = PID 
        2 = Users 
        4 = Type IP 
        7 = Protocol 
        8 =  IP
*/

#[derive(Debug, Clone)]
pub struct Ports {
    process: String, 
    pid: String,
    user: String,
    type_ip: String,
    protocol: String,
    ip: String,
} 

pub fn get_open_connection()  -> Result<Vec<Ports>, Box<dyn std::error::Error>> {
    let output = Command::new("lsof")
        .arg("-i")
        .arg("-P")
        .arg("-n")  
        .output()?;
    
    let result = String::from_utf8_lossy(&output.stdout);
    
    let mut output: Vec<Ports> = Vec::new();
    for line in result.lines().skip(1) {  
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 4 {

            output.push(
                Ports { 
                    process: parts[0].to_string(), 
                    pid: parts[1].to_string(), 
                    user: parts[2].to_string(), 
                    type_ip: parts[4].to_string(), 
                    protocol: parts[7].to_string(), 
                    ip: parts[8].to_string() });   
        }
    }

    Ok(output)
}