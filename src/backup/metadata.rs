use bincode::{Encode, Decode};

#[derive(Encode, Decode, Debug)]
struct FileInfo {
    relative_path: String,
    hash: String,
    timestamp: u64,
    size: u64,
}

#[derive(Encode, Decode, Debug)]
struct BackupMetadata {
    files: Vec<FileInfo>,
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


/* 
fn main() {
    let metadata = BackupMetadata {
        files: vec![
            FileInfo {
                relative_path: "documents/file.txt".to_string(),
                hash: "abc123".to_string(),
                timestamp: 1730943600,
                size: 1024,
            }
        ],
    };

    save_metadata(&metadata).unwrap();
    let loaded = load_metadata().unwrap();
    println!("{:?}", loaded);
}
*/