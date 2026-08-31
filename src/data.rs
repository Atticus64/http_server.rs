use crate::utils::IDManager;

pub struct User {
  name: String,
  id: i64
}

impl User {
    pub fn new(name: &str) -> Self {
        let manager = IDManager::new();
        let user = User {
            name: name.to_string(),
            id: manager.get_id()
        };

        user
    }

    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json += "{ ";
        json += format!("\"name\": \"{}\", ", self.name).as_str();
        json += format!("\"id\": {}", self.id).as_str();
        json += "}";

        json
    }
}


