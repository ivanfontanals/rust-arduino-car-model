use anyhow::Result;

use crate::domain::ports::incoming::BinaryConverter;
use crate::domain::ports::outgoing::BinaryReader;

pub struct FileBinaryConverter<R>
where
    R: BinaryReader + Send,
{
    reader: R,
}

impl<R> FileBinaryConverter<R>
where
    R: BinaryReader + Send,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> BinaryConverter for FileBinaryConverter<R>
where
    R: BinaryReader + Send,
{
    fn write<T>(&self, _output: &str) -> Result<()> {
        Ok(())
    }

    fn convert<T>(&self) -> Result<Vec<T>> {
        self.reader.read()
    }
}
