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

#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::{thread_rng, Fill};

    pub fn generate_random_byte_array(size: usize) -> Vec<u8> {
        let mut rng = thread_rng();
        let mut r = vec![0 as u8; size];
        r.as_mut_slice().try_fill(&mut rng).unwrap_or_default();
        r
    }

    pub fn generate_random_object(size: usize) -> Object {
        Object::from(generate_random_byte_array(size))
    }

    #[test]
    fn object_write_should_produce_identical_data() {
        let size: usize = 32 * 1024;
        let src = generate_random_byte_array(size);
        let mut dst = Object::new(size);
        let _ = io::copy(&mut src.as_slice(), &mut dst).unwrap_or_default();
        assert_eq!(
            &src, &dst.data,
            "Write to Object failed to produce identical data."
        );
    }

    #[test]
    fn object_reader_read_should_output_all_data() {
        let size: usize = 32 * 1024;
        let src = generate_random_object(size);
        let mut reader = ObjectReader::new(&src);
        let mut dst = io::sink();
        let read = io::copy(&mut reader, &mut dst).unwrap_or_default();
        assert_eq!(
            size as u64, read,
            "ObjectReader failed to output all data. Expected {} bytes, got {}.",
            size, read
        );
    }

    #[test]
    fn object_reader_read_should_output_identical_data() {
        let size: usize = 32 * 1024;
        let src = generate_random_object(size);
        let mut reader = ObjectReader::new(&src);
        let mut dst = Vec::with_capacity(size);
        let _ = io::copy(&mut reader, &mut dst).unwrap_or_default();
        assert_eq!(
            &src.data, &dst,
            "Read for ObjectReader failed to produce identical data."
        );
    }
}
