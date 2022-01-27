use anyhow::Result;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait BinaryReader: Send {
    fn read<T>(&self) -> Result<Vec<T>>;
}
