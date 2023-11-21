use crate::app::{
    model::{
        request::{CreateOne, ToggleState},
        todo,
    },
    repo_impl::mysql_impl::TodoImpl,
    service,
};

use super::model::error;

#[tauri::command]
pub fn get_list(
    state: tauri::State<'_, service::Todo<TodoImpl>>,
) -> Result<Vec<todo::Todo>, String> {
    match state.get_list() {
        Ok(list) => Ok(list),
        Err(err) => match err {
            error::Error::TodoNotFound => Err(String::from("todo not found")),
        },
    }
}

#[tauri::command]
pub fn get_archive(
    state: tauri::State<'_, service::Todo<TodoImpl>>,
) -> Result<Vec<todo::Todo>, String> {
    match state.get_archive() {
        Ok(list) => Ok(list),
        Err(err) => match err {
            error::Error::TodoNotFound => Err(String::from("todo not found")),
        },
    }
}

#[tauri::command]
pub fn get_recent(state: tauri::State<'_, service::Todo<TodoImpl>>) -> Result<todo::Todo, String> {
    match state.get_recent() {
        Ok(todo) => Ok(todo),
        Err(err) => match err {
            error::Error::TodoNotFound => Err(String::from("todo not found")),
        },
    }
}

#[tauri::command]
pub fn create_todo(
    state: tauri::State<'_, service::Todo<TodoImpl>>,
    label: String,
) -> Result<todo::Todo, String> {
    match state.create_one(CreateOne { label }) {
        Ok(created) => Ok(created),
        Err(err) => match err {
            error::Error::TodoNotFound => Err(String::from("todo not found")),
        },
    }
}

#[tauri::command]
pub fn complete_todo(
    id: u32,
    state: tauri::State<'_, service::Todo<TodoImpl>>,
) -> Result<todo::Todo, String> {
    match state.toggle_state(ToggleState { id, state: true }) {
        Ok(completed) => Ok(completed),
        Err(err) => match err {
            error::Error::TodoNotFound => Err(String::from("todo not found")),
        },
    }
}

#[tauri::command]
pub fn uncomplete_todo(
    id: u32,
    state: tauri::State<'_, service::Todo<TodoImpl>>,
) -> Result<todo::Todo, String> {
    match state.toggle_state(ToggleState { id, state: false }) {
        Ok(completed) => Ok(completed),
        Err(err) => match err {
            error::Error::TodoNotFound => Err(String::from("todo not found")),
        },
    }
}
