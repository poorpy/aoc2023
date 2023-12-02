use anyhow::Result;

pub trait Solution {
    fn first(self: &Self, path: &str) -> Result<()>;
    fn second(self: &Self, path: &str) -> Result<()>;
}
