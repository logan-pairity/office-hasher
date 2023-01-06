use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use csv::Reader;
use rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OfficeLine {
    character: String,
    line: String,
    episode: usize,
    season: usize
}

unsafe impl Send for OfficeLine {}
unsafe impl Sync for OfficeLine {}


#[derive(Debug)]
pub struct OfficeLineManifest{
    lines: Vec<OfficeLine>,
}

unsafe impl Send for OfficeLineManifest {}
unsafe impl Sync for OfficeLineManifest {}

impl OfficeLineManifest{
    pub fn new(path: &str) -> Self {
        let mut lines: Vec<OfficeLine> = Vec::new();
        let mut reader = Reader::from_path(path).unwrap();

        for result in reader.records() {
            let record = result.unwrap();
            lines.push(
                OfficeLine {
                    season: record.get(1).unwrap().parse().unwrap(),
                    episode: record.get(2).unwrap().parse().unwrap(),
                    line: record.get(4).unwrap().to_string(),
                    character: record.get(5).unwrap().to_string(),
                }
            )
        }

        OfficeLineManifest { lines }
    }

    pub fn get_line_from_hash<T: Hash> (&self, item: &T) -> &OfficeLine {
        let mut s = DefaultHasher::new();
        item.hash(&mut s);
        let index = s.finish();

        // the indexing is a gnarly hack basically `index` is a u64, but
        // we need a usize for the `.get`
        //
        // So we modulo is by the max number for usize (dynamic to the system)
        // cast to a u64 so its binary-operator compatible with `index`.
        // Then the result is cast down to a usize and modulo'd by the max
        // length of the lines collection.
        //
        // On 64-bit systems this line is equivalent to `index % self.lines.len()`,
        // but this accounts for lower bit systems so the distribution of hashes is
        // still even across the number-space.
        &self.lines.get((index % usize::MAX as u64) as usize % self.lines.len()).unwrap()
    }
}