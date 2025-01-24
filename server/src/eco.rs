use axum::{extract::Query, Json};
use serde::Deserialize;
use ucui_eco::{lookup_eco_from_name, Eco};

#[derive(Deserialize)]
pub struct Lookup {
    term: String,
}

pub async fn lookup_eco(Query(lookup): Query<Lookup>) -> Json<Vec<Eco>> {
    Json(lookup_eco_from_name(&lookup.term))
}
