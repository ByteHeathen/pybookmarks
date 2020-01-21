use pyo3::prelude::*;

use libbookmarks::BookMarksApi;
use libbookmarks::NewFolder;

#[pyclass]
pub struct Folder {
    pub(crate) id: i32,
    #[pyo3(get, set)]
    pub(crate) label: String,
    #[pyo3(get, set)]
    pub(crate) parent: Option<i32>
}

#[pymethods]
impl Folder {

    #[staticmethod]
    fn all(database_path: Option<String>) -> PyResult<Vec<Folder>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(api.all_folders()?.iter().map(|item| {
            Folder {
                id: item.id,
                label: item.label.clone(),
                parent: item.parent.clone()
            }
        }).collect())
    }

    #[staticmethod]
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
    fn create(label: String, parent: Option<i32>, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let raw_folder = NewFolder { label, parent };
        api.create_folder(raw_folder)?;
        Ok(())
    }

    fn save(&self, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let mut folder = api.get_folder(self.id)?;
        folder.label = self.label.clone();
        folder.parent = self.parent.clone();
        folder.save(&api)?;
        Ok(())
    }
}
