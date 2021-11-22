use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value); //Format: key <tab> value\n
    std::fs::write("public/kv.db", contents).unwrap();

    let database = Database::new().expect("Creating DB failed");
}

struct Database {
    map: HashMap<String, String>
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("public/kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt databse");
            map.insert(key.to_owned(), value.to_owned());
        }
        // parse the string
        // populate our map
        Ok(Database {
            map: map,
        })
    }
}