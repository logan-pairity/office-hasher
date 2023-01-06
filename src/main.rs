#[macro_use] extern crate rocket;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::Rng;

use csv::Reader;
use rocket::State;
use rocket::serde::{json::Json, Serialize, Deserialize};


// I dont want to install Sqlite rn.
struct PoorMansDB {
    list_of_lines: OfficeLineManifest,
}

unsafe impl Send for PoorMansDB {}
unsafe impl Sync for PoorMansDB {}


#[derive(Debug, Clone, Serialize)]
struct OfficeLine {
    character: String,
    line: String,
    episode: usize,
    season: usize
}

unsafe impl Send for OfficeLine {}
unsafe impl Sync for OfficeLine {}


#[derive(Debug)]
struct OfficeLineManifest{
    lines: Vec<OfficeLine>,
}

unsafe impl Send for OfficeLineManifest {}
unsafe impl Sync for OfficeLineManifest {}

impl OfficeLineManifest{
    fn new(path: &str) -> Self {
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

    fn get_line_from_hash<T: Hash> (&self, item: &T) -> &OfficeLine {
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

    fn get_first_list(&self) -> &OfficeLine {
        self.lines.first().expect("Lines not initialized!")
    }

}

#[derive(Debug, Clone, Deserialize)]
struct HashRequest {
    payload: String
}

#[post("/hash", format = "json", data = "<hashable_payload>")]
fn hash_payload(state: &State<PoorMansDB>, hashable_payload: Json<HashRequest>) -> Json<OfficeLine> {
    let line_to_serve = state.list_of_lines.get_line_from_hash(&hashable_payload.payload);
    Json(line_to_serve.clone())
}

#[get("/random")]
fn random_response(state: &State<PoorMansDB>) -> Json<OfficeLine> {
    let mut rng = rand::thread_rng();
    Json(state.list_of_lines.get_line_from_hash(&rng.gen::<usize>().to_string()).clone())
}


#[get("/")]
fn index() -> &'static str {
    "Server is running..."
}



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let lines = OfficeLineManifest::new(&"src/resources/the-office-lines - scripts.csv");

    let config = PoorMansDB {
        list_of_lines: lines
    };

    let _rocket = rocket::build()
        .mount("/", routes![index, random_response, hash_payload])
        .manage(config)
        .launch().await?;

    Ok(())
}
