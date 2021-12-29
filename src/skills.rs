use std::fs::File;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Skills {
    skilltype: String,
    skills: String,
}

impl Skills {
    pub fn default() -> Vec<Self> {
        let jsonfile = File::open("json/skills.json").unwrap();
        serde_json::from_reader(jsonfile).unwrap()
    }
}
