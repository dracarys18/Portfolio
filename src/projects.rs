use std::fs::File;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Projects {
    name: String,
    git: String,
    description: String,
}

impl Projects {
    pub(crate) fn default() -> Vec<Self> {
        let file = File::open("json/projects.json").unwrap();
        serde_json::from_reader(file).unwrap()
    }
}
