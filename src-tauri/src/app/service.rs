use crate::app::{
    model::{error, request, todo},
    repo,
};

pub struct Todo<T: repo::Todo> {
    r: T,
}

impl<T: repo::Todo> Todo<T> {
    pub fn new(r: T) -> Self {
        Todo { r }
    }
}

impl<T: repo::Todo> Todo<T> {
    pub fn get_list(&self) -> error::Result<Vec<todo::Todo>> {
        self.r.get_list()
    }
    pub fn get_archive(&self) -> error::Result<Vec<todo::Todo>> {
        self.r.get_archive()
    }
    pub fn get_recent(&self) -> error::Result<todo::Todo> {
        self.r.get_recent()
    }
    pub fn create_one(&self, params: request::CreateOne) -> error::Result<todo::Todo> {
        self.r.save(&mut todo::Todo {
            label: params.label,
            ..Default::default()
        })
    }
    pub fn toggle_state(&self, params: request::ToggleState) -> error::Result<todo::Todo> {
        let todo = self.r.get_by_id(params.id)?;
        self.r.save(&mut todo::Todo {
            id: Some(params.id),
            label: todo.label,
            state: params.state,
        })
    }
}
