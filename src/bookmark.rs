use pyo3::prelude::*;

use libbookmarks::BookMarksApi;
use libbookmarks::NewBookMark;

use crate::Tag;

/// BookMark object
#[pyclass]
pub struct BookMark {
    /// Unique identifier for this bookmark.
    pub(crate) id: i32,
    /// the url this bookmark links to.
    #[pyo3(get, set)]
    pub(crate) url: String,
    /// The label used when displaying this bookmark.
    #[pyo3(get, set)]
    pub(crate) label: Option<String>,
    /// The folder this bookmark belongs to.
    #[pyo3(get, set)]
    pub(crate) folder: Option<i32>
}

#[pymethods]
impl BookMark {

    #[staticmethod]
    /// List all bookmarks tracted by libbookmarks
    ///
    /// @parameter database_path: Option<String>
    fn all(database_path: Option<String>) -> PyResult<Vec<BookMark>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(api.all_bookmarks()?.iter().map(|item| {
            BookMark {
                id: item.id,
                url: item.url.clone(),
                label: item.label.clone(),
                folder: item.folder
            }
        }).collect())
    }

    #[staticmethod]
    /// Find a particular bookmark by id.
    ///
    /// @parameter = id: i32
    /// @parameter = database_path: Option<String>
    fn find(id: i32, database_path: Option<String>) -> PyResult<BookMark> {
        let api = BookMarksApi::new(database_path)?;
        let raw_bookmark = api.get_bookmark(id)?;
        Ok(BookMark {
            id: raw_bookmark.id,
            url: raw_bookmark.url,
            label: raw_bookmark.label,
            folder: raw_bookmark.folder
        })
    }

    #[staticmethod]
    /// Remove a particular `BookMark` by its id.
    ///
    /// @parameter = id: i32
    /// @parameter = database_path: Option<String>
    fn remove_bookmark(id: i32, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        api.remove_bookmark(id)?;
        Ok(())
    }

    /// Delete this particular `BookMark`.
    ///
    /// @parameter = database_path: Option<String>
    fn delete(&self, database_path: Option<String>) -> PyResult<()> {
        BookMark::remove_bookmark(self.id, database_path)?;
        Ok(())
    }

    #[staticmethod]
    /// Create a new BookMark object.
    ///
    /// @parameter = url: String
    /// @parameter = label: Option<String>
    /// @parameter = folder: Option<i32>
    /// @parameter = database_path: Option<String>
    fn create(url: String, label: Option<String>, folder: Option<i32>, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let raw_bookmark = NewBookMark { url, label, folder };
        api.create_bookmark(raw_bookmark)?;
        Ok(())
    }

    /// Assign a new tag to this bookmark.
    ///
    /// @parameter = id: i32
    /// @parameter = database_path: Option<String>
    fn assign_tag(&self, id: i32, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let self_bookmark = api.get_bookmark(self.id)?;
        self_bookmark.assign_tag(&api, id)?;
        Ok(())
    }

    /// Remove a tag from this bookmark.
    ///
    /// @parameter = id: i32
    /// @parameter = database_path: Option<String>
    fn remove_tag(&self, id: i32, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let self_bookmark = api.get_bookmark(self.id)?;
        self_bookmark.remove_tag(&api, id)?;
        Ok(())
    }

    /// Save changes in this bookmark.
    ///
    /// This must be called after modifing the
    /// `url`, `label` or `folder` fields.
    ///
    /// @parameter = database_path: Option<String>
    fn save(&self, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let mut bookmark = api.get_bookmark(self.id)?;
        bookmark.url = self.url.clone();
        bookmark.label = self.label.clone();
        bookmark.folder = self.folder;
        bookmark.save(&api)?;
        Ok(())
    }

    /// List all tags associated with this bookmark.
    ///
    /// @parameter = database_path: Option<String>
    fn tags(&self, database_path: Option<String>) -> PyResult<Vec<Tag>> {
        let api = BookMarksApi::new(database_path)?;
        let bookmark = api.get_bookmark(self.id)?;
        Ok(bookmark.tags(&api)?.iter().map(|item| {
            Tag {
                id: item.id,
                label: item.label.clone(),
                color: item.color.clone()
            }
        }).collect())
    }
}
