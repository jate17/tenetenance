use bincode::{Encode, Decode};

#[derive(Encode, Decode, Debug)]
pub struct FileInfo {
    pub relative_path: String,
    pub hash: String,
    pub timestamp_modified: u64,
    pub timestamp_backup: u64,
}

#[derive(Encode, Decode, Debug)]
pub struct BackupMetadata {
    pub files: Vec<FileInfo>,
}

pub fn save_metadata(metadata: &BackupMetadata) -> std::io::Result<()> {
    let bytes = bincode::encode_to_vec(metadata, bincode::config::standard())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write("backup.bincode", bytes)
}

pub fn load_metadata() -> std::io::Result<BackupMetadata> {
    let bytes = std::fs::read("backup.bincode")?;
    let (metadata, _): (BackupMetadata, usize) = bincode::decode_from_slice(&bytes, bincode::config::standard())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(metadata)
}

