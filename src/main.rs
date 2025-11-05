use sysinfo::{self, System, Disks};





fn main() {
    
    let mut sys = System::new_all();

    let disks = Disks::new_with_refreshed_list();
    println!("{:?}", disks.len());
    for disk in &disks{
        let d = conv_bytes_storage(disk.available_space());
        let t = conv_bytes_storage(disk.total_space());
        let name = disk.name();
        println!("{name:?}");
        println!("{d:.2}");
        println!("{t:.2}");
    }

}


/// Convert bytes to Gigabytes 
/// 
///  # Parms
/// - bytes: u64 
/// 
/// # Return 
/// - Velues in Gibaytes -> u32 

fn conv_bytes_ram(bytes: u64) -> u32 {
    (bytes as f64 / 1_073_741_824.0).floor() as u32
}


fn conv_bytes_storage(bytes: u64) -> f64 {
    (bytes as f64 / 1_000_000_000.0)
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

fn ram_check(mut sys: System) -> Vec<u32> {    
    sys.refresh_all();


    let gb_tot : u32 = conv_bytes_ram(sys.total_memory());
    let gb_used : u32 = conv_bytes_ram(sys.used_memory());
    let swap_tot : u32 = conv_bytes_ram(sys.total_swap());
    let swap_used : u32 = conv_bytes_ram(sys.used_swap());

    vec![gb_tot, gb_used, swap_tot, swap_used]
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

fn storage_check() -> Vec<(String, f64, f64)>{
    let disks = Disks::new_with_refreshed_list();
    let mut output: Vec<(String, f64,f64)> = Vec::new();
    for disk in &disks{
        let available_space = conv_bytes_storage(disk.available_space());
        let total_sapce = conv_bytes_storage(disk.total_space());
        let name = disk.name().to_string_lossy().to_string();
        output.push((name, total_sapce, available_space));
    }

    output
}