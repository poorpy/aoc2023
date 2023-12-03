use anyhow::Result;

pub trait Solution {
    fn first(&self, path: &str) -> Result<()>;
    fn second(&self, path: &str) -> Result<()>;
}
