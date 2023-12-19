use color_eyre::Result;
use uuid::Uuid;

use crate::dbcontext::DbContext;

pub enum CurrentScreen {
    Main,
    Adding,
}

#[derive(Debug)]
pub struct TodoItem {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
}

pub struct App {
    pub input: String,
    pub todo_cursor_pos: isize,
    pub todo_items: Vec<TodoItem>,
    pub current_screen: CurrentScreen,
    pub currently_adding: bool,
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            todo_cursor_pos: -1,
            todo_items: Vec::<TodoItem>::new(),
            current_screen: CurrentScreen::Main,
            currently_adding: false,
        }
    }
    pub fn save_todo_item(&mut self) -> Result<()> {
        let new_item = TodoItem {
            id: Uuid::new_v4(),
            name: self.input.clone(),
            completed: false,
        };

        DbContext::store_todo_item(&new_item)?;

        self.todo_items.push(new_item);

        self.input = String::new();
        self.currently_adding = false;

        Ok(())
    }
    pub fn delete_todo_item(&mut self) -> Result<()> {
        let item = self.todo_items.remove(self.todo_cursor_pos as usize);

        DbContext::delete_todo_item(&item)?;

        if self.todo_cursor_pos == self.todo_items.len() as isize {
            Self::move_list_cursor_up(self)
        }

        Ok(())
    }
    pub fn load_todo_items(&mut self) -> Result<()> {
        self.todo_items = DbContext::load_all_todo_items()?;

        Ok(())
    }
    pub fn move_list_cursor_up(&mut self) {
        if self.todo_cursor_pos > -1 {
            self.todo_cursor_pos -= 1;
        }
    }
    pub fn move_list_cursor_down(&mut self) {
        if self.todo_items.is_empty() {
            return;
        }

        if self.todo_cursor_pos < self.todo_items.len().saturating_sub(1) as isize {
            self.todo_cursor_pos += 1;
        }
    }
    pub fn mark_item_complete(&mut self) -> Result<()> {
        let item = self.todo_items.get_mut(self.todo_cursor_pos as usize).unwrap();

        item.completed = !item.completed;

        DbContext::update_todo_item(item)?;

        Ok(())
    }
}