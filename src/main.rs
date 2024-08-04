use chrono::Local;

pub mod database;

fn main() {
    let db = database::Database::new("./test.db");

    let users = db.get_users();

    for user in users {
        println!("User id: {}, name: {}", user.id, user.name);
    }

    let loans = db.get_loans();
    for loan in &loans {
        println!(
            "{} loans {} ({}) from {} to {}",
            loan.user.name,
            loan.instance.product.name,
            loan.instance.identifier,
            loan.date_start.format("%Y-%m-%d %H:%m"),
            loan.date_end.format("%Y-%m-%d %H:%m"),
        );
    }

    println!("{}", Local::now())
}
