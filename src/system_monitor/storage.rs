use sysinfo::{self, System, Disks};
use crate::utils::{conv_bytes_storage};
use crate::logs::{Logs};

///
/// Struct for Storage Info 
/// 
/// - Name -> String 
/// - Total Space -> f64 
/// - Available Space -> f64
/// 
#[derive(Debug, Clone)]
pub struct Storage{
    pub name: String, 
    pub total_space: f64, 
    pub available_space: f64
}


/// Check Storage info
/// 
/// # Parms
/// None 
/// 
/// # Return 
/// 
/// Tuple inside the vector, tuple struct is String, f64, f64
/// The two f64 is in GB

pub fn storage_check() -> Vec<Storage> {
    let disks = Disks::new_with_refreshed_list();
    
    let mut output: Vec<(Storage)> = Vec::new();
    
    for disk in &disks{
        let available_space = conv_bytes_storage(disk.available_space());
        let total_sapce = conv_bytes_storage(disk.total_space());
        let name = disk.name().to_string_lossy().to_string();
        output.push(
            Storage{
                name: name, 
                total_space: total_sapce, 
                available_space: available_space
            
        });
    }
    let _ = Logs::trace("Storage check");
    output
}