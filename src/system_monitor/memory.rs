use sysinfo::{self, System};
use crate::utils::{conv_bytes_ram};


///
/// Struct for RAM info 
/// 
/// gb_tot -> u32
/// gb_used -> u32
/// swap_tot -> u32
/// swap_used -> u32
/// 
#[derive(Debug, Clone)]
pub struct Ram {
    gb_tot: u32, 
    gb_used: u32, 
    swap_tot: u32, 
    swap_used: u32, 
}


/// Get the RAM info
/// # Parms
/// - 'sys': instance 'sysinfo::System'
/// 
/// # Return 
/// Four u32 inside the vector
///
/// - Total Memory (GB)
/// - Used Memory (GB)
/// - Total Swap (GB)
/// - Used Swap (GB)

pub fn ram_check(mut sys: System) -> Ram {    

    let gb_tot : u32 = conv_bytes_ram(sys.total_memory());
    let gb_used : u32 = conv_bytes_ram(sys.used_memory());
    let swap_tot : u32 = conv_bytes_ram(sys.total_swap());
    let swap_used : u32 = conv_bytes_ram(sys.used_swap());

    Ram{
        gb_tot, 
        gb_used, 
        swap_tot, 
        swap_used
    }   
}