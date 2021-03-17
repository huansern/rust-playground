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
