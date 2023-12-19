use color_eyre::Result;
use rusqlite::Connection;

use crate::app::TodoItem;

pub struct DbContext;

impl DbContext {
    fn try_connect() -> Result<Connection> {
        Ok(Connection::open("./todo.db")?)
    }
    pub fn try_init_db() -> Result<()> {
        let conn = Self::try_connect()?;

        conn.execute(
            "CREATE TABLE todo (
                id        INTEGER PRIMARY KEY,
                todo_id   UUID BLOB NOT NULL,
                name      TEXT NOT NULL,
                completed BOOL
            )",
            ()).ok(); // Don't need to worry about failed init

        Ok(())
    }
    pub fn store_todo_item(todo_item: &TodoItem) -> Result<()> {
        let conn = Self::try_connect()?;

        conn.execute("INSERT INTO todo (todo_id, name, completed)
                            VALUES (?1, ?2, ?3)",
                     (&todo_item.id, &todo_item.name, &todo_item.completed))?;

        Ok(())
    }
    pub fn load_all_todo_items() -> Result<Vec<TodoItem>> {
        let conn = Self::try_connect()?;

        let mut stmt = conn.prepare("SELECT todo_id, name, completed FROM todo")?;

        let todo_iter = stmt.query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                name: row.get(1)?,
                completed: row.get(2)?,
            })
        })?;

        let mut todo_items: Vec<TodoItem> = Vec::<TodoItem>::new();

        for item in todo_iter {
            todo_items.push(item?);
        }

        Ok(todo_items)
    }
    pub fn update_todo_item(todo_item: &TodoItem) -> Result<()> {
        let conn = Self::try_connect()?;

        conn.execute("UPDATE todo
                          SET (name, completed) =
                              (?2, ?3)
                          WHERE todo_id = ?1",
                     (&todo_item.id, &todo_item.name, &todo_item.completed))?;

        Ok(())
    }
    pub fn delete_todo_item(todo_item: &TodoItem) -> Result<()> {
        let conn = Self::try_connect()?;

        conn.execute("DELETE FROM todo
                          WHERE todo_id = ?1",
                     [&todo_item.id])?;

        Ok(())
    }
}