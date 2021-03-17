use std::io::{Read, Write};
use std::{cmp, io};

pub struct Object {
    data: Vec<u8>,
}

impl Object {
    pub fn new(size: usize) -> Self {
        Object {
            data: Vec::with_capacity(size),
        }
    }
}

impl<T: Into<Vec<u8>>> From<T> for Object {
    fn from(data: T) -> Self {
        Object { data: data.into() }
    }
}

impl Write for Object {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct ObjectReader<'a> {
    read: usize,
    data: &'a [u8],
}

impl<'a> ObjectReader<'a> {
    pub fn new(object: &'a Object) -> Self {
        ObjectReader {
            read: 0,
            data: &object.data[..],
        }
    }
}

impl<'a> From<&'a [u8]> for ObjectReader<'a> {
    fn from(data: &'a [u8]) -> Self {
        ObjectReader { read: 0, data }
    }
}

impl<'a> From<&'a Object> for ObjectReader<'a> {
    fn from(object: &'a Object) -> Self {
        ObjectReader {
            read: 0,
            data: &object.data[..],
        }
    }
}

impl<'a> Read for ObjectReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.data.len() <= self.read {
            return Ok(0);
        }
        let data = &self.data[self.read..];
        let copy_size = cmp::min(data.len(), buf.len());
        if copy_size > 0 {
            let src = &data[..copy_size];
            let dst = &mut buf[..copy_size];
            dst.copy_from_slice(src);
            self.read += copy_size;
        }
        Ok(copy_size)
    }
}
