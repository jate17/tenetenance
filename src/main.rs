use sysinfo::{self, System, Disks,  RefreshKind, CpuRefreshKind};





fn main() {
    
    let mut sys = System::new_all();

    sys.refresh_all();


}

/// Struct for che Single Core 
/// 
/// # Struct 
/// - index: usize 
/// - usage: f32 
/// - frequency: u64
/// 
#[derive(Debug, Clone)]
struct CoreInfo {
    index: usize,
    usage: f32,
    frequency: u64
}
///
/// Struct for the CPU 
/// 
/// # Struct 
/// - brand: String 
/// - vendor: String 
/// - frequency: u64 
/// - cores: Muliple CoreInfo struct inside the vector 
#[derive(Debug, Clone)]
struct CPU{
    brand: String, 
    vendor: String, 
    frequency: u64,
    cores: Vec<CoreInfo>
}

/// Get info form every single core 
/// 
/// # Parms 
/// - sys: Instance form sysinfo::system 
/// 
/// # Retrun 
/// Return multiple CoreInfo struct inside the vector

fn get_cpu_cores(sys: System) -> Vec<CoreInfo> {
    sys.cpus()
        .iter()
        .enumerate()
        .map(|(i, cpu)| CoreInfo{
            index: i,
            usage: cpu.cpu_usage(),
            frequency: cpu.frequency()
        })
        .collect()
}

/// Get CPU info 
/// 
/// # Parms 
/// - None
/// 
/// # Return
/// Return struct CPU

fn cpu_check() -> CPU {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );

    sys.refresh_cpu_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_frequency();
    sys.refresh_cpu_all();

     let cpu = sys.cpus().first()
        .expect("CPU not found");

    let brand = cpu.brand().to_string();
    let vendor = cpu.vendor_id().to_string();
    let frequency = cpu.frequency();

    let cores = get_cpu_cores(sys);

    CPU {
        brand,
        vendor,
        frequency,
        cores
    }

}


/// Convert bytes to Gigabytes -> Only RAM
/// 
///  # Parms
/// - bytes: u64 
/// 
/// # Return 
/// - Velues in Gibaytes -> u32 

fn conv_bytes_ram(bytes: u64) -> u32 {
    (bytes as f64 / 1_073_741_824.0).floor() as u32
}

/// Convert bytes to Gigabytes -> Only Storage
///  # Parms
/// - bytes: u64 
/// 
/// # Return 
/// - Velues in Gibaytes -> f64 

fn conv_bytes_storage(bytes: u64) -> f64 {
    (bytes as f64 / 1_000_000_000.0)
}


#[derive(Debug, Clone)]
struct RAM {
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

fn ram_check(mut sys: System) -> RAM {    

    let gb_tot : u32 = conv_bytes_ram(sys.total_memory());
    let gb_used : u32 = conv_bytes_ram(sys.used_memory());
    let swap_tot : u32 = conv_bytes_ram(sys.total_swap());
    let swap_used : u32 = conv_bytes_ram(sys.used_swap());

    RAM{
        gb_tot, 
        gb_used, 
        swap_tot, 
        swap_used
    }   
}

#[derive(Debug, Clone)]
struct Storage(String, f64, f64);

/// Check Storage info
/// 
/// # Parms
/// None 
/// 
/// # Return 
/// 
/// Tuple inside the vector, tuple struct is String, f64, f64
/// The two f64 is in GB

fn storage_check() -> Vec<Storage> {
    let disks = Disks::new_with_refreshed_list();
    
    let mut output: Vec<(Storage)> = Vec::new();
    
    for disk in &disks{
        let available_space = conv_bytes_storage(disk.available_space());
        let total_sapce = conv_bytes_storage(disk.total_space());
        let name = disk.name().to_string_lossy().to_string();
        let storage = Storage(name, total_sapce, available_space);
        output.push(storage);
    }

    output
}