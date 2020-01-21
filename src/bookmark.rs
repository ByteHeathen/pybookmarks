use pyo3::prelude::*;

use libbookmarks::BookMarksApi;
use libbookmarks::NewBookMark;

use crate::Tag;

#[pyclass]
pub struct BookMark {
    pub(crate) id: i32,
    pub(crate) url: String,
    pub(crate) label: Option<String>,
    pub(crate) folder: Option<i32>
}

#[pymethods]
impl BookMark {

    #[staticmethod]
    fn all(database_path: Option<String>) -> PyResult<Vec<BookMark>> {
        let api = BookMarksApi::new(database_path)?;
        Ok(api.all_bookmarks()?.iter().map(|item| {
            BookMark {
                id: item.id,
                url: item.url.clone(),
                label: item.label.clone(),
                folder: item.folder.clone()
            }
        }).collect())
    }

    #[staticmethod]
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
    fn create(url: String, label: Option<String>, folder: Option<i32>, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let raw_bookmark = NewBookMark { url, label, folder };
        api.create_bookmark(raw_bookmark)?;
        Ok(())
    }

    fn assign_tag(&self, id: i32, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let self_bookmark = api.get_bookmark(self.id)?;
        self_bookmark.assign_tag(&api, id)?;
        Ok(())
    }

    fn remove_tag(&self, id: i32, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let self_bookmark = api.get_bookmark(self.id)?;
        self_bookmark.remove_tag(&api, id)?;
        Ok(())
    }

    fn save(&self, database_path: Option<String>) -> PyResult<()> {
        let api = BookMarksApi::new(database_path)?;
        let mut bookmark = api.get_bookmark(self.id)?;
        bookmark.url = self.url.clone();
        bookmark.label = self.label.clone();
        bookmark.folder = self.folder.clone();
        bookmark.save(&api)?;
        Ok(())
    }

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
