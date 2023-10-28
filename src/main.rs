use std::io::Error;
use std::io::Read;

use std::env;

use std::str::FromStr;

use std::collections::HashMap;

use std::fs::write;
use std::fs::OpenOptions;


fn main() {
    let action = env::args().nth(1).expect("Please specify an action");
    let item = env::args().nth(2).expect("Please specify an item");

    println!("{}, {}", action, item);

    let mut todo = Todo::new().expect("Initialization of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved."),
            Err(err) => println!("An error occurred: {}", err),
        }
    } else if action == "complete" {
        match todo.complete_todo(&item) {
            None => println!("'{}' is not present in the list.", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved."),
                Err(err) => println!("An error occurred: {}", err),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let map: HashMap<String, bool> = file_contents
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    fn complete_todo(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
    fn save(self) -> Result<(), Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        write("db.txt", content)
    }
}
