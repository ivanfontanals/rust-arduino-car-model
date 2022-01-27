use anyhow::Result;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait BinaryConverter: Send {
    fn write<T>(&self, output: &str) -> Result<()>;
    fn convert<T>(&self) -> Result<Vec<T>>;
}
