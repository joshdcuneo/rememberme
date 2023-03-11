pub mod query;

use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Entry {
    pub name: String,
    pub description: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct CreateEntry {
    pub name: String,
    pub description: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct UpdateEntry {
    pub name: String,
    pub description: String,
}
