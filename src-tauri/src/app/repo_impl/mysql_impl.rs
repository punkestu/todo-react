use crate::app::{
    model::{
        error::{self, Result},
        todo,
    },
    repo,
};
use sqlx::{Row, SqlitePool};
use tokio::runtime::Runtime;

pub struct TodoImpl {
    pool: SqlitePool,
    rt: Runtime,
}

impl TodoImpl {
    pub fn new(pool: SqlitePool, rt: Runtime) -> Self {
        TodoImpl { pool, rt }
    }
}

impl repo::Todo for TodoImpl {
    fn get_recent(&self) -> Result<todo::Todo> {
        self.rt.block_on(async {
            if let Ok(row) = sqlx::query("SELECT * FROM todo WHERE state = 0 ORDER BY id DESC")
                .fetch_one(&self.pool)
                .await
            {
                let todo = todo::Todo {
                    id: Some(row.get::<u32, _>("id")),
                    label: row.get::<String, _>("label"),
                    state: row.get::<bool, _>("state"),
                };
                Ok(todo)
            } else {
                Err(crate::app::model::error::Error::TodoNotFound)
            }
        })
    }
    fn get_list(&self) -> Result<Vec<todo::Todo>> {
        self.rt.block_on(async {
            let rows = sqlx::query("SELECT * FROM todo WHERE state = 0")
                .fetch_all(&self.pool)
                .await
                .unwrap();
            let todos = rows
                .iter()
                .map(|row| todo::Todo {
                    id: Some(row.get::<u32, _>("id")),
                    label: row.get::<String, _>("label"),
                    state: row.get::<bool, _>("state"),
                })
                .collect::<Vec<todo::Todo>>();
            Ok(todos)
        })
    }
    fn get_archive(&self) -> Result<Vec<todo::Todo>> {
        self.rt.block_on(async {
            let rows = sqlx::query("SELECT * FROM todo WHERE state = 1")
                .fetch_all(&self.pool)
                .await
                .unwrap();
            let todos = rows
                .iter()
                .map(|row| todo::Todo {
                    id: Some(row.get::<u32, _>("id")),
                    label: row.get::<String, _>("label"),
                    state: row.get::<bool, _>("state"),
                })
                .collect::<Vec<todo::Todo>>();
            Ok(todos)
        })
    }
    fn get_by_id(&self, id: u32) -> Result<todo::Todo> {
        self.rt.block_on(async {
            if let Ok(row) = sqlx::query("SELECT * FROM todo WHERE id=?")
                .bind(id)
                .fetch_one(&self.pool)
                .await
            {
                let todo = todo::Todo {
                    id: Some(row.get::<u32, _>("id")),
                    label: row.get::<String, _>("label"),
                    state: row.get::<bool, _>("state"),
                };
                Ok(todo)
            } else {
                Err(error::Error::TodoNotFound)
            }
        })
    }
    fn save(&self, todo: &mut todo::Todo) -> Result<todo::Todo> {
        match todo.id {
            Some(id) => self.rt.block_on(async {
                sqlx::query("UPDATE todo SET label=?, state=? WHERE id=?")
                    .bind(&todo.label)
                    .bind(todo.state)
                    .bind(id)
                    .execute(&self.pool)
                    .await
                    .unwrap();
                Ok(todo.to_owned())
            }),
            None => self.rt.block_on(async {
                let result = sqlx::query("INSERT INTO todo (id, label) VALUES (NULL, ?)")
                    .bind(&todo.label)
                    .execute(&self.pool)
                    .await
                    .unwrap();
                todo.id = Some(result.last_insert_rowid() as u32);
                Ok(todo.to_owned())
            }),
        }
    }
}
