use crate::in_memory::object::{Object, ObjectReader};
use std::collections::HashMap;

pub struct Bucket {
    objects: HashMap<String, Object>,
}

impl Bucket {
    pub fn new() -> Self {
        Bucket {
            objects: HashMap::<String, Object>::new(),
        }
    }

    pub fn names(&self) -> Vec<&str> {
        self.objects
            .keys()
            .map(|k| k.as_ref())
            .collect::<Vec<&str>>()
    }

    pub fn exist(&self, name: &str) -> bool {
        self.objects.contains_key(name)
    }

    pub fn put(&mut self, name: &str, object: Object) {
        self.objects.insert(name.into(), object);
    }

    pub fn get(&self, name: &str) -> Option<ObjectReader> {
        match self.objects.get(name) {
            None => None,
            Some(obj) => Some(ObjectReader::from(obj)),
        }
    }

    pub fn remove(&mut self, name: &str) -> Option<Object> {
        self.objects.remove(name)
    }
}
