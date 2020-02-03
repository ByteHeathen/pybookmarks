use pyo3::prelude::*;

use libbookmarks::BookMarksApi;
use libbookmarks::NewBookMark;
use libbookmarks::models::BookMark as RawBookMark;

use crate::Tag;

/// BookMark object
#[pyclass]
pub struct BookMark {
    /// Unique identifier for this bookmark.
    #[pyo3(get)]
    pub(crate) id: i32,
    /// the url this bookmark links to.
    #[pyo3(get, set)]
    pub(crate) url: String,
    /// The label used when displaying this bookmark.
    #[pyo3(get, set)]
    pub(crate) label: Option<String>,
    /// The folder this bookmark belongs to.
    #[pyo3(get, set)]
    pub(crate) folder: Option<i32>,
    #[pyo3(get, set)]
    pub(crate) starred: bool
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
                folder: item.folder,
                starred: item.starred
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
            folder: raw_bookmark.folder,
            starred: raw_bookmark.starred
        })
    }

    #[staticmethod]
    /// Find all bookmarks with a particular url.
    ///
    /// @parameter = url: String
    /// @parameter = database_path: Option<String>
    fn find_url(url: String, database_path: Option<String>) -> PyResult<Vec<BookMark>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(RawBookMark::find_url(&api, &url)?.into_iter().map(|raw_bookmark| {
            BookMark {
                id: raw_bookmark.id,
                url: raw_bookmark.url,
                label: raw_bookmark.label,
                folder: raw_bookmark.folder,
                starred: raw_bookmark.starred
            }
        }).collect())
    }

    #[staticmethod]
    /// Find all bookmarks with a particular label.
    ///
    /// @parameter = label: String
    /// @parameter = database_path: Option<String>
    fn find_label(label: String, database_path: Option<String>) -> PyResult<Vec<BookMark>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(RawBookMark::find_label(&api, label.as_ref())?.into_iter().map(|raw_bookmark| {
            BookMark {
                id: raw_bookmark.id,
                url: raw_bookmark.url,
                label: raw_bookmark.label,
                folder: raw_bookmark.folder,
                starred: raw_bookmark.starred
            }
        }).collect())
    }

    #[staticmethod]
    /// Find all bookmarks with a certain folder.
    ///
    /// @parameter = folder: i32
    /// @parameter = database_path: Option<String>
    fn find_folder(folder: i32, database_path: Option<String>) -> PyResult<Vec<BookMark>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(RawBookMark::find_folder(&api, folder)?.into_iter().map(|raw_bookmark| {
            BookMark {
                id: raw_bookmark.id,
                url: raw_bookmark.url,
                label: raw_bookmark.label,
                folder: raw_bookmark.folder,
                starred: raw_bookmark.starred
            }
        }).collect())
    }

    #[staticmethod]
    /// Find all bookmarks that are starred.
    ///
    /// @parameter = folder: bool
    /// @parameter = database_path: Option<String>
    fn find_starred(starred: bool, database_path: Option<String>) -> PyResult<Vec<BookMark>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(RawBookMark::find_starred(&api, starred)?.into_iter().map(|raw_bookmark| {
            BookMark {
                id: raw_bookmark.id,
                url: raw_bookmark.url,
                label: raw_bookmark.label,
                folder: raw_bookmark.folder,
                starred: raw_bookmark.starred
            }
        }).collect())
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
    /// @parameter = starred: bool
    /// @parameter = database_path: Option<String>
    fn create(url: String, label: Option<String>, folder: Option<i32>, starred: bool, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let raw_bookmark = NewBookMark { url, label, folder, starred };
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
        bookmark.starred = self.starred;
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
