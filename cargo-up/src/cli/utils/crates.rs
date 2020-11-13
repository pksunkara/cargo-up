use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Krate {
    pub name: String,
    pub max_version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Vers {
    pub num: String,
    pub yanked: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Upgrader {
    #[serde(rename = "crate")]
    pub krate: Krate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Versions {
    pub versions: Vec<Vers>,
}
