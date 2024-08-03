pub mod database;

fn main() {
    let db = database::Database::new("./test.db");

    let users = db.get_users();

    for user in users {
        println!("User id: {}, name: {}", user.id, user.name);
    }

    for loan in db.get_loans() {
        println!(
            "{} loans {}({}) from {} to {}",
            loan.user.name,
            loan.instance.product.name,
            loan.instance.identifier,
            loan.date_start,
            loan.date_end,
        );
    }
}
