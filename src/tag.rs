use pyo3::prelude::*;
use libbookmarks::BookMarksApi;
use libbookmarks::NewTag;

#[pyclass]
pub struct Tag {
    pub(crate) id: i32,
    #[pyo3(get, set)]
    pub(crate) label: String,
    #[pyo3(get, set)]
    pub(crate) color: Option<String>
}

#[pymethods]
impl Tag {

    #[staticmethod]
    /// Create a new `Tag` that can be used to organize bookmarks.
    ///
    /// @parameter = label: String
    /// @parameter = color: Option<String>
    fn create(label: String, color: Option<String>, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let raw_tag = NewTag { label, color };
        api.create_tag(raw_tag)?;
        Ok(())
    }

    #[staticmethod]
    fn find(id: i32, database_path: Option<String>) -> PyResult<Tag> {
        let api = BookMarksApi::new(database_path)?;
        let raw_tag = api.get_tag(id)?;
        Ok(Tag {
            id: raw_tag.id,
            label: raw_tag.label,
            color: raw_tag.color
        })
    }

    #[staticmethod]
    fn all(database_path: Option<String>) -> PyResult<Vec<Tag>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(api.all_tags()?.iter().map(|item| {
            Tag {
                id: item.id,
                label: item.label.clone(),
                color: item.color.clone()
            }
        }).collect())
    }

    fn save(&self, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let mut tag = api.get_tag(self.id)?;
        tag.label = self.label.clone();
        tag.color = self.color.clone();
        tag.save(&api)?;
        Ok(())
    }

    fn bookmarks(&self, database_path: Option<String>) -> PyResult<Vec<crate::BookMark>> {
        let api = BookMarksApi::new(database_path)?;
        let tag = api.get_tag(self.id)?;
        Ok(tag.bookmarks(&api)?.iter().map(|item| {
            crate::BookMark {
                id: item.id,
                url: item.url.clone(),
                label: item.label.clone(),
                folder: item.folder.clone()
            }
        }).collect())
    }
}
