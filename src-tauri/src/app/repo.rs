use crate::app::model::{error, todo};

pub trait Todo {
    fn get_list(&self) -> error::Result<Vec<todo::Todo>>;
    fn get_archive(&self) -> error::Result<Vec<todo::Todo>>;
    fn get_recent(&self) -> error::Result<todo::Todo>;
    fn get_by_id(&self, id: u32) -> error::Result<todo::Todo>;
    fn save(&self, todo: &mut todo::Todo) -> error::Result<todo::Todo>;
}
