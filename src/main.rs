use rusqlite::{Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

fn main() {
    let conn = Connection::open("./test.db").unwrap();

    let mut statement = conn.prepare("select * from user;").unwrap();
    let user_iter = statement
        .query_map([], |row| {
            Ok(User {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for user in user_iter {
        println!(
            "User id: {}, name: {}",
            user.as_ref().unwrap().id,
            user.as_ref().unwrap().name
        );
    }
}
