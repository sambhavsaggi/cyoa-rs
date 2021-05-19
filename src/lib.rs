//! A Crate to facilitate the creation of Choose-Your-Own-Adventure games in a text-based format

mod datastruct;
use crate::datastruct::Path;

/// The "State" of the game, including config, current path, and history
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Game {
    pub config: datastruct::Game,
    current_path: u16,
    history: Vec<u16>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Game {
    /// Sets the game state and history to something else based on user interaction
    pub fn jump(&mut self, path: u16) {
        if !self.config.check_path(&path) {
            panic!("Path doesn't exist");
        }
        self.current_path = path;
        self.history.push(path)
    }

    /// Gets the current path the game is on
    pub fn get_path(&self) -> &Path {
        &self.config.get_path(self.current_path)
    }

    // TODO: Implement
    /// Exports the save to a string so others can load and replay (WIP)
    pub fn export_save(&self) -> String {
        String::new()
    }

    /// Imports a string to the State to load a history
    pub fn import_save(&mut self, save: &String) {
        self.history.clear();
        for item in save.split(",") {
            self.history.push(item.parse::<u16>().unwrap());
        }
    }

    // Expensive but I don't expect it to be used much, so it's probably fine
    /// Go back (removes item from history)
    pub fn backtrack(&mut self) {
        if self.history.len() >= 1 {
            let prev = self.history.last().unwrap();
            self.current_path = *prev;
        }
    }
    /// Loads the game and returns a new State
    pub fn new(data: &String) -> Game {
        Game {
            config: serde_json::from_str(data).unwrap(),
            current_path: 0,
            history: Vec::<u16>::new(),
        }
    }
}
