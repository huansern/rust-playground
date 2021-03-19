use crate::error::{Error, Kind as ErrorKind, Result};
use crate::in_memory::bucket::Bucket;
use crate::in_memory::object::Object;
use crate::store::Store;
use std::collections::HashMap;
use std::io;
use std::io::Read;

pub struct InMemoryStore {
    buckets: HashMap<String, Bucket>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            buckets: HashMap::<String, Bucket>::new(),
        }
    }

    pub fn insert_or_replace_object(
        &mut self,
        bucket: &str,
        name: &str,
        reader: &mut Box<dyn Read>,
        replace: bool,
    ) -> Result<()> {
        self.create_bucket(bucket);
        let container = match self.buckets.get_mut(bucket) {
            None => return Err(Error::new(ErrorKind::BucketNotFound, bucket, name)),
            Some(bucket) => bucket,
        };
        if !replace && container.exist(name) {
            return Err(Error::new(ErrorKind::ObjectAlreadyExist, bucket, name));
        }
        let mut obj = Object::new(0);
        match io::copy(reader, &mut obj) {
            Err(_) => return Err(Error::new(ErrorKind::IO, bucket, name)),
            Ok(_) => {}
        }
        container.put(name.into(), obj);
        Ok(())
    }
}

impl Store for InMemoryStore {
    fn list_buckets(&self) -> Vec<&str> {
        self.buckets
            .keys()
            .map(|k| k.as_ref())
            .collect::<Vec<&str>>()
    }

    fn create_bucket(&mut self, name: &str) {
        if !self.buckets.contains_key(name) {
            self.buckets.insert(name.into(), Bucket::new());
        }
    }

    fn delete_bucket(&mut self, name: &str) -> Result<()> {
        self.buckets.remove(name.into());
        Ok(())
    }

    fn list_objects(&self, bucket: &str) -> Option<Vec<&str>> {
        match self.buckets.get(bucket) {
            None => None,
            Some(bucket) => Some(bucket.names()),
        }
    }

    fn insert_object(
        &mut self,
        bucket: &str,
        name: &str,
        reader: &mut Box<dyn Read>,
    ) -> Result<()> {
        self.insert_or_replace_object(bucket, name, reader, false)
    }

    fn put_object(&mut self, bucket: &str, name: &str, reader: &mut Box<dyn Read>) -> Result<()> {
        self.insert_or_replace_object(bucket, name, reader, true)
    }

    fn get_object<'a>(&'a self, bucket: &str, name: &str) -> Option<Box<dyn Read + 'a>> {
        let bucket = match self.buckets.get(bucket) {
            None => return None,
            Some(bucket) => bucket,
        };
        match bucket.get(name) {
            None => None,
            Some(obj) => Some(Box::new(obj)),
        }
    }

    fn remove_object(&mut self, bucket: &str, name: &str) -> Result<()> {
        let bucket = match self.buckets.get_mut(bucket) {
            None => return Err(Error::new(ErrorKind::BucketNotFound, bucket, name)),
            Some(bucket) => bucket,
        };
        bucket.remove(name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &'static [u8] = b"";

    fn get_reader() -> Box<dyn Read> {
        Box::new(EMPTY)
    }

    fn get_bucket_names() -> Vec<&'static str> {
        vec!["Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto"]
    }

    fn get_object_names() -> Vec<&'static str> {
        vec!["Io", "Europa", "Ganymede", "Callisto"]
    }

    fn get_store(buckets: &Vec<&str>) -> InMemoryStore {
        let mut store = InMemoryStore::new();
        for bucket in buckets {
            store.create_bucket(bucket);
        }
        store
    }

    fn get_populated_store(bucket: &str, objects: &Vec<&str>) -> InMemoryStore {
        let mut store = InMemoryStore::new();
        for object in objects {
            let _ = store.insert_object(bucket, object, &mut get_reader());
        }
        store
    }

    #[test]
    fn store_list_buckets_should_return_all_bucket_names() {
        let mut buckets = get_bucket_names();
        let store = get_store(&buckets);
        let mut output = store.list_buckets();
        buckets.sort();
        output.sort();
        assert_eq!(
            buckets, output,
            "InMemoryStore failed to list all bucket names."
        );
    }

    #[test]
    fn store_create_bucket_should_add_bucket_when_bucket_did_not_exist() {
        let mut store = InMemoryStore::new();
        let bucket = "Earth";
        store.create_bucket(bucket);
        assert!(
            store.list_buckets().contains(&bucket),
            "InMemoryStore failed to create new bucket."
        );
    }

    #[test]
    fn store_delete_bucket_should_delete_existing_bucket() {
        let mut store = get_store(&get_bucket_names());
        let bucket = "Pluto";
        let _ = store.delete_bucket(bucket);
        assert!(
            !store.list_buckets().contains(&bucket),
            "InMemoryStore failed to delete bucket."
        );
    }

    #[test]
    fn store_list_objects_should_list_all_object_names_in_bucket() {
        let bucket = "Jupiter";
        let mut objects = get_object_names();
        let store = get_populated_store(bucket, &objects);
        let mut output = store.list_objects(bucket).unwrap_or_default();
        objects.sort();
        output.sort();
        assert_eq!(
            objects, output,
            "InMemoryStore failed to list all object names in a bucket."
        );
    }

    #[test]
    fn store_insert_object_should_add_object_when_object_did_not_exist() {
        let bucket = "Earth";
        let object = "Moon";
        let mut store = InMemoryStore::new();
        assert!(
            store
                .insert_object(bucket, object, &mut get_reader())
                .is_ok(),
            "InMemoryStore failed to insert new object."
        );
    }

    #[test]
    fn store_insert_object_should_not_add_object_when_object_already_exist() {
        let bucket = "Jupiter";
        let objects = get_object_names();
        let object = objects[0].clone();
        let mut store = get_populated_store(bucket, &objects);
        assert!(
            store
                .insert_object(bucket, object, &mut get_reader())
                .is_err(),
            "InMemoryStore insert duplicated object."
        );
    }

    #[test]
    fn store_put_object_should_replace_existing_object() {
        let bucket = "Jupiter";
        let objects = get_object_names();
        let object = objects[0].clone();
        let mut store = get_populated_store(bucket, &objects);
        assert!(
            store.put_object(bucket, object, &mut get_reader()).is_ok(),
            "InMemoryStore failed to replace an existing object."
        );
    }

    #[test]
    fn store_get_object_should_return_existing_object() {
        let bucket = "Earth";
        let object = "Moon";
        let mut store = InMemoryStore::new();
        let _ = store.put_object(bucket, object, &mut get_reader());
        assert!(
            store.get_object(bucket, object).is_some(),
            "InMemoryStore failed to return existing object."
        );
    }

    #[test]
    fn store_remove_object_should_delete_existing_object() {
        let bucket = "Jupiter";
        let object = "Callisto";
        let mut store = get_populated_store(bucket, &get_object_names());
        let _ = store.remove_object(bucket, object);
        assert!(
            store.get_object(bucket, object).is_none(),
            "InMemoryStore failed to remove existing object."
        );
    }
}
