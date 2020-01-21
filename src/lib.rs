
use pyo3::prelude::*;

mod tag;
pub use tag::Tag;

mod folder;
pub use folder::Folder;

mod bookmark;
pub use bookmark::BookMark;

/// libbookmarks is a Rust library for managing bookmarks statically.
#[pymodule]
fn libpybookmarks(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tag>()?;
    m.add_class::<Folder>()?;
    m.add_class::<BookMark>()?;
    Ok(())
}
