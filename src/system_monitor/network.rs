use sysinfo::{self, Networks};
use crate::utils::{conv_bytes_to_mb};

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
