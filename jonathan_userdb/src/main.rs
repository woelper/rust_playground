use std::{collections::HashMap, sync::Mutex, path::Path, fs::File};

use rocket::State;
#[macro_use] extern crate rocket;


struct SimpleDB {
    messages: Mutex<HashMap<String, Vec<String>>>
}

impl Default for SimpleDB {
    fn default() -> Self {
        SimpleDB { messages: Mutex::new(HashMap::default()) }
    }
}

impl SimpleDB {
    fn save(&self, p: &str) {
        let f = File::create(p).unwrap();
        // strange syntax ahead: you need to dereference the Mutex with *, but serde wants a ref (&)
        serde_json::to_writer_pretty(f, &*self.messages.lock().unwrap()).unwrap();
    }

    fn load(&self, p: &str) {
        let f = File::open(p).unwrap();
        *self.messages.lock().unwrap() = serde_json::from_reader(f).unwrap();
    }
}


#[get("/")]
fn index(state: &State<SimpleDB>) -> String {
    format!("{:#?}", state.messages.lock().unwrap())
}

#[get("/save")]
fn save(state: &State<SimpleDB>) {
    state.save("mydb")
}

#[get("/get/<user>")]
fn get(user: &str, state: &State<SimpleDB>) -> String {
    match state.messages.lock().unwrap().get(user) {
        Some(messages) => format!("{:#?}", messages),
        None =>format!("No messages for {}!", user)
    }
}

#[get("/send/<user>/<message>")]
fn send(user: &str, message: &str, state: &State<SimpleDB>)  {
    let mut messages = state.messages.lock().unwrap();
    // see https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
    let e = messages.entry(user.to_string());
    e.or_default().push(message.to_string())
}

#[launch]
fn rocket() -> _ {
    let db = SimpleDB::default();
    // add a message for johann so he does not feel as lonely
    // Since this is using a mutex, it needs locking
    db.load("mydb");
    db.messages.lock().unwrap().insert("johann@woelper.de".to_string(),vec!["Hi dude".to_string()]);

    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![get])
    .mount("/", routes![send])
    .mount("/", routes![save])
    .manage(db)
}


