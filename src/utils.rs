
/// Convert bytes to Gigabytes -> Only RAM
/// 
///  # Parms
/// - bytes: u64 
/// 
/// # Return 
/// - Velues in Gibaytes -> u32 

pub fn conv_bytes_ram(bytes: u64) -> u32 {
    (bytes as f64 / 1_073_741_824.0).floor() as u32
}

/// Convert bytes to Gigabytes -> Only Storage
///  # Parms
/// - bytes: u64 
/// 
/// # Return 
/// - Velues in Gibaytes -> f64 

pub fn conv_bytes_storage(bytes: u64) -> f64 {
    (bytes as f64 / 1_000_000_000.0)
}


/// 
/// Convert bytes to Mb -> Network use
/// 
/// # Parms 
/// - bytes: u64
/// 
/// # Return 
/// - Float 64 in MB
pub fn conv_bytes_to_mb_net(bytes: u64) -> f64{
    bytes as f64 / 1_000_000.0
}