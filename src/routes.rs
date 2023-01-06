use rocket::State;
use rand::Rng;
use rocket::serde::{json::Json, Deserialize};

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct HashRequest {
    payload: String
}

#[post("/hash", format = "json", data = "<hashable_payload>")]
pub fn hash_payload(state: &State<PoorMansDB>, hashable_payload: Json<HashRequest>) -> Json<OfficeLine> {
    let line_to_serve = state.list_of_lines.get_line_from_hash(&hashable_payload.payload);
    Json(line_to_serve.clone())
}

#[get("/random")]
pub fn random_response(state: &State<PoorMansDB>) -> Json<OfficeLine> {
    let mut rng = rand::thread_rng();
    Json(state.list_of_lines.get_line_from_hash(&rng.gen::<usize>().to_string()).clone())
}

#[get("/")]
pub fn index() -> &'static str {
    "Server is running..."
}

