use sysinfo::{self, System};





fn main() {
    
    let mut sys = System::new_all();

    let ram: Vec<u32> = ram_check(sys);
    

    println!("Memoria: {:?} bytes ", ram);


}


/// Convert bytes to Gigabytes 
/// 
///  # Parms
/// - bytes: u64 
/// 
/// # Return 
/// - Velues in Gibaytes -> u32 

fn conv_bytes(bytes: u64) -> u32 {
    (bytes as f64 / 1_073_741_824.0).floor() as u32
}



/// Get the RAM info
/// # Parms
/// - 'sys': instance 'sysinfo::System'
/// 
/// # Return 
/// Vector u32
/// - Total Memory (GB)
/// - Used Memory (GB)
/// - Total Swap (GB)
/// - Used Swap (GB)

fn ram_check(mut sys: System) -> Vec<u32> {    
    sys.refresh_all();


    let gb_tot : u32 = conv_bytes(sys.total_memory());
    let gb_used : u32 = conv_bytes(sys.used_memory());
    let swap_tot : u32 = conv_bytes(sys.total_swap());
    let swap_used : u32 = conv_bytes(sys.used_swap());

    vec![gb_tot, gb_used, swap_tot, swap_used]
}   
