use std::io::Error;
use std::env;
use std::collections::HashMap;
use std::fs::write;


fn main() {
    let action = env::args().nth(1).expect("Please specify an action");
    let item = env::args().nth(2).expect("Please specify an item");

    println!("{}, {}", action, item);

    let mut todo = Todo {
        map: HashMap::new(),
    };
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved."),
            Err(err) => println!("An error occurred: {}", err),
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
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
