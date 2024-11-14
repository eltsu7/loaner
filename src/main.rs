use crate::database::{Database, LoanQueryParams};
use chrono_tz::Europe::Helsinki;

pub mod database;
pub mod test_database;

fn main() {
    let db_name = "test.db";
    // let db = Database::new(db_name);

    let db = test_database::initialize_test_database(Some(db_name));

    let user_uuid = db.get_users()[0].uuid.clone();

    let now = chrono::Utc::now().with_timezone(&Helsinki);
    let later = now + chrono::Duration::days(1);

    let product = db.get_products(None)[0].clone();
    let instance_uuid = db.get_instances(Some(product.uuid))[0].uuid.clone();

    let result = db
        .add_loan(user_uuid, vec![instance_uuid], now, later)
        .unwrap();

    dbg!(result);

    let loans = db.get_loans(LoanQueryParams::new());
    println!("Loan count: {}", loans.len());
    for loan in &loans {
        println!("Loan: {:?}", loan);
    }
}
