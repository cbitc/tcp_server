use crate::service::Service;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Router {
    table: HashMap<String, Box<dyn Service>>,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            table: HashMap::default(),
        }
    }
}

impl Router {
    pub fn from_map(map: HashMap<String, Box<dyn Service>>) -> Self {
        Self { table: map }
    }

    pub fn get(&self, path: &str) -> Option<&Box<dyn Service>> {
        self.table.get(path)
    }

    pub fn insert(&mut self, (path, service): (&str, Box<dyn Service>)) {
        self.table.insert(path.to_string(), service);
    }
}
