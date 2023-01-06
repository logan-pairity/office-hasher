use crate::models::OfficeLineManifest;

// I dont want to install Sqlite rn.
pub struct PoorMansDB {
    pub list_of_lines: OfficeLineManifest,
}

unsafe impl Send for PoorMansDB {}

unsafe impl Sync for PoorMansDB {}