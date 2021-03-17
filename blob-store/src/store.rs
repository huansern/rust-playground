use crate::error::Result;
use std::io::Read;

pub trait Store {
    fn list_buckets(&self) -> Vec<&str>;
    fn create_bucket(&mut self, name: &str);
    fn delete_bucket(&mut self, name: &str) -> Result<()>;
    fn list_objects(&self, bucket: &str) -> Option<Vec<&str>>;
    fn insert_object(&mut self, bucket: &str, name: &str, reader: &mut Box<dyn Read>)
        -> Result<()>;
    fn put_object(&mut self, bucket: &str, name: &str, reader: &mut Box<dyn Read>) -> Result<()>;
    fn get_object<'a>(&'a self, bucket: &str, name: &str) -> Option<Box<dyn Read + 'a>>;
    fn remove_object(&mut self, bucket: &str, name: &str) -> Result<()>;
}
