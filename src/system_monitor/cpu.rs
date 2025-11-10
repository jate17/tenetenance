use sysinfo::{self, System,RefreshKind, CpuRefreshKind};
use crate::logs::{Logs};


/// Struct for che Single Core 
/// 
/// # Struct 
/// - index: usize 
/// - usage: f32 
/// - frequency: u64
/// 
#[derive(Debug, Clone)]
pub struct CoreInfo {
    pub index: usize,
    pub usage: f32,
    pub frequency: u64
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
pub struct Cpu{
    pub brand: String, 
    pub vendor: String, 
    pub frequency: u64,
    pub cores: Vec<CoreInfo>
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

pub fn cpu_check() -> Cpu {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
    );

    sys.refresh_cpu_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_frequency();
    sys.refresh_cpu_all();

     let cpu = sys.cpus().first()
        .expect({
            let _ = Logs::error("CPU not found");
            "CPU not found"
        });

    let brand = cpu.brand().to_string();
    let vendor = cpu.vendor_id().to_string();
    let frequency = cpu.frequency();

    let cores = get_cpu_cores(sys);
    
    let _ = Logs::trace("CPU Check");

    Cpu {
        brand,
        vendor,
        frequency,
        cores
    }

}

