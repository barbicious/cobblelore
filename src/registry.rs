use std::ops::Index;
use std::path::Path;

pub trait Registry<'a>: Index<usize> + Index<&'a str> {
    fn from_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>
    where
        Self: Sized;
}