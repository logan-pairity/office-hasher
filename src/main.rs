#[macro_use] extern crate rocket;

use crate::models::OfficeLineManifest;

mod routes;
mod models;
mod db;


mod prelude {
    pub use crate::routes::*;
    pub use crate::db::*;
    pub use crate::models::*;
}

use prelude::*;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let lines = OfficeLineManifest::new(&"src/resources/the-office-lines - scripts.csv");

    let config = db::PoorMansDB {
        list_of_lines: lines
    };

    let _rocket = rocket::build()
        .mount("/", routes![index, random_response, hash_payload])
        .manage(config)
        .launch().await?;

    Ok(())
}
