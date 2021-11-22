use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key: String = arguments.next().unwrap();
    let value: String = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let mut database: Database = Database::new().expect("Creating DB failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        let contents = std::fs::read_to_string("public/kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt databse");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map: map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(&mut self) {
        self.flush = true;
        do_flush(&self);
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents: String = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}