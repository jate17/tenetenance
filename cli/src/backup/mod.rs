pub mod backup;
pub mod metadata;

pub use backup::{make_backup, checksum, verify_file_backup, get_current_timestamp, get_last_modified, exclude, sync_backup};
pub use metadata::{FileInfo, BackupMetadata, save_metadata, load_metadata};