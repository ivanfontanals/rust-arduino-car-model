use crate::domain::ports::outgoing::BinaryReader;
use anyhow::Result;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct FileBinaryReader<P>
where
    P: AsRef<Path> + Send,
{
    path: P,
}

impl<P> FileBinaryReader<P>
where
    P: AsRef<Path> + Send,
{
    pub fn new(input: P) -> Self {
        Self { path: input }
    }
}

/**
 *
#[async_trait(?Send)]
pub trait DataBinaryReader: Send {
    /// This method will decode a binary file into a text file, with the data that has been read in the Arduino Car
    fn write<T, P: AsRef<Path>>(&self, input: P, output: &str) -> Result<()>;
    fn read<T, P: AsRef<Path>>(&self, path: P) -> Result<Vec<T>>;
}
 */
impl<P> BinaryReader for FileBinaryReader<P>
where
    P: AsRef<Path> + Send,
{
    fn read<T>(&self) -> Result<Vec<T>> {
        let path = self.path.as_ref();
        let struct_size = ::std::mem::size_of::<T>();
        let num_bytes = fs::metadata(path)?.len() as usize;
        let num_structs = num_bytes / struct_size;
        let mut reader = BufReader::new(File::open(path)?);
        let mut r = Vec::<T>::with_capacity(num_structs);
        unsafe {
            let buffer = std::slice::from_raw_parts_mut(r.as_mut_ptr() as *mut u8, num_bytes);
            reader.read_exact(buffer)?;
            r.set_len(num_structs);
        }
        Ok(r)
    }
}
