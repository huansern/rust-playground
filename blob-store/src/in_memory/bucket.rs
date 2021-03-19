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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::in_memory::object::tests::generate_random_byte_array;
    use std::io;

    fn get_names() -> Vec<&'static str> {
        vec!["Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto"]
    }

    fn get_bucket(names: &Vec<&str>) -> Bucket {
        let mut bucket = Bucket::new();
        for name in names {
            bucket.put(name, Object::new(0));
        }
        bucket
    }

    fn get_filled_bucket() -> Bucket {
        get_bucket(&get_names())
    }

    #[test]
    fn bucket_names_should_return_all_object_names() {
        let mut names = get_names();
        let bucket = get_bucket(&names);
        let mut output = bucket.names();
        names.sort();
        output.sort();
        assert_eq!(names, output, "Bucket did not return all object names.");
    }

    #[test]
    fn bucket_exist_should_return_true_when_object_exist() {
        let bucket = get_filled_bucket();
        assert!(bucket.exist("Saturn"));
    }

    #[test]
    fn bucket_exist_should_return_false_when_object_did_not_exist() {
        let bucket = Bucket::new();
        assert!(!bucket.exist("Earth"));
    }

    #[test]
    fn bucket_put_should_add_when_object_did_not_exist() {
        let mut bucket = Bucket::new();
        let name = "Earth";
        bucket.put(name, Object::new(0));
        assert!(bucket.exist(name), "Bucket failed to put new object.");
    }

    #[test]
    fn bucket_put_should_replace_when_object_already_exist() {
        let mut bucket = get_filled_bucket();
        let name = "Jupiter";
        let size = 1024;
        let input = generate_random_byte_array(size);
        let clone = Object::from(input.clone());
        bucket.put(name, clone);
        let empty = Object::new(0);
        let mut reader = bucket
            .get(name)
            .unwrap_or_else(|| ObjectReader::new(&empty));
        let mut output = Vec::new();
        let _ = io::copy(&mut reader, &mut output);
        assert_eq!(input, output, "Bucket failed to replace existing object.");
    }

    #[test]
    fn bucket_get_should_return_existing_object() {
        let mut bucket = Bucket::new();
        let name = "Earth";
        bucket.put(name, Object::new(0));
        assert!(
            bucket.get(name).is_some(),
            "Bucket did not return an existing object."
        );
    }

    #[test]
    fn bucket_get_should_return_none_when_object_did_not_exist() {
        let bucket = Bucket::new();
        assert!(
            bucket.get("").is_none(),
            "Bucket returned a non-existent object."
        );
    }

    #[test]
    fn bucket_remove_should_delete_existing_object() {
        let mut bucket = get_filled_bucket();
        let name = "Pluto";
        bucket.remove(name);
        assert!(
            bucket.get(name).is_none(),
            "Bucket failed to remove object."
        )
    }
}
