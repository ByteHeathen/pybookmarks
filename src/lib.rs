
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod tag;
pub use tag::Tag;

mod folder;
pub use folder::Folder;

mod bookmark;
pub use bookmark::BookMark;

#[pyfunction]
/// List all Folders with no parent folder.
fn root_folders(database_path: Option<String>) -> PyResult<Vec<Folder>> {
    use libbookmarks::BookMarksApi;

    let api = BookMarksApi::new(database_path)?;
    Ok(api.root_folders()?.iter().map(|item| {
        Folder {
            id: item.id,
            label: item.label.clone(),
            parent: item.parent
        }
    }).collect())
}

/// libbookmarks is a Rust library for managing bookmarks statically.
#[pymodule]
fn pybookmarks(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tag>()?;
    m.add_class::<Folder>()?;
    m.add_class::<BookMark>()?;
    m.add_wrapped(wrap_pyfunction!(root_folders))?;
    Ok(())
}
