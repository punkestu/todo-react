use super::{
    model::{
        self,
        request::{CreateOne, ToggleState},
    },
    repo_impl::mysql_impl::TodoImpl,
    service,
};

#[tauri::command]
pub fn get_list(
    state: tauri::State<'_, service::Todo<TodoImpl>>,
) -> Result<Vec<model::todo::Todo>, String> {
    Ok(state.get_list().unwrap())
}

#[tauri::command]
pub fn get_archive(
    state: tauri::State<'_, service::Todo<TodoImpl>>,
) -> Result<Vec<model::todo::Todo>, String> {
    Ok(state.get_archive().unwrap())
}

#[tauri::command]
pub fn get_recent(state: tauri::State<'_, service::Todo<TodoImpl>>) -> Option<model::todo::Todo> {
    match state.get_recent() {
        Ok(todo) => Some(todo),
        Err(_) => None,
    }
}

#[tauri::command]
pub fn create_todo(
    state: tauri::State<'_, service::Todo<TodoImpl>>,
    label: String,
) -> model::todo::Todo {
    state.create_one(CreateOne { label }).unwrap()
}

#[tauri::command]
pub fn complete_todo(
    id: u32,
    state: tauri::State<'_, service::Todo<TodoImpl>>,
) -> model::todo::Todo {
    state.toggle_state(ToggleState { id, state: true }).unwrap()
}

#[tauri::command]
pub fn uncomplete_todo(
    id: u32,
    state: tauri::State<'_, service::Todo<TodoImpl>>,
) -> model::todo::Todo {
    state
        .toggle_state(ToggleState { id, state: false })
        .unwrap()
}
