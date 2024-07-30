use rusqlite::Connection;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

fn get_users(conn: Connection) -> Vec<User> {
    let mut statement = conn.prepare("select * from user;").unwrap();
    let user_iter = statement
        .query_map([], |row| {
            Ok(User {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
            })
        })
        .unwrap();

    let mut users = Vec::new();

    for user in user_iter {
        users.push(user.unwrap());
    }
    return users;
}

fn main() {
    let conn = Connection::open("./test.db").unwrap();

    let users = get_users(conn);

    for user in users {
        println!("User id: {}, name: {}", user.id, user.name);
    }
}
