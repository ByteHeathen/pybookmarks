use pyo3::prelude::*;

use libbookmarks::BookMarksApi;
use libbookmarks::NewFolder;

/// Folder used to organize bookmarks
#[pyclass]
pub struct Folder {
    /// Unique identifier for this folder.
    #[pyo3(get)]
    pub(crate) id: i32,
    /// Label used when displaying this folder.
    #[pyo3(get, set)]
    pub(crate) label: String,
    /// The parent folder of this folder.
    #[pyo3(get, set)]
    pub(crate) parent: Option<i32>
}

#[pymethods]
impl Folder {

    #[staticmethod]
    /// List all folders tracted by libbookmarks.
    ///
    /// @parameter = database_path: Option<String>
    fn all(database_path: Option<String>) -> PyResult<Vec<Folder>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(api.all_folders()?.iter().map(|item| {
            Folder {
                id: item.id,
                label: item.label.clone(),
                parent: item.parent
            }
        }).collect())
    }

    #[staticmethod]
    /// Find a particular folder.
    ///
    /// @parameter = id: i32
    /// @parameter = database_path: Option<String>
    fn find(id: i32, database_path: Option<String>) -> PyResult<Folder> {
        let api = BookMarksApi::new(database_path)?;
        let raw_tag = api.get_folder(id)?;
        Ok(Folder {
            id: raw_tag.id,
            label: raw_tag.label,
            parent: raw_tag.parent
        })
    }

    #[staticmethod]
    /// Remove a particular `Folder` by its id.
    ///
    /// @parameter = id: i32
    /// @parameter = database_path: Option<String>
    fn remove_folder(id: i32, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        api.remove_folder(id)?;
        Ok(())
    }

    /// Delete this particular `Folder`.
    ///
    /// @parameter = database_path: Option<String>
    fn delete(&self, database_path: Option<String>) -> PyResult<()> {
        Folder::remove_folder(self.id, database_path)?;
        Ok(())
    }

    #[staticmethod]
    /// Create a new bookmark object.
    ///
    /// @parameter = label: String
    /// @parameter = parent: Option<i32>
    /// @parameter = database_path: Option<String>
    fn create(label: String, parent: Option<i32>, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let raw_folder = NewFolder { label, parent };
        api.create_folder(raw_folder)?;
        Ok(())
    }

    /// Save changes in this folder.
    ///
    /// This must be called after modifing the
    /// `label` or `parent` fields.
    ///
    /// @parameter = database_path: Option<String>
    fn save(&self, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let mut folder = api.get_folder(self.id)?;
        folder.label = self.label.clone();
        folder.parent = self.parent;
        folder.save(&api)?;
        Ok(())
    }
}
