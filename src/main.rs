use crate::database::{Database, LoanQueryParams};

pub mod database;
pub mod test_database;

fn main() {
    let db_name = "test.db";
    let db = Database::new(db_name);

    let loans = db.get_loans(LoanQueryParams::new());
    println!("Loan count: {}", loans.len());
    for loan in &loans {
        println!(
            "{} loans \t{} ({}) \t from {} \tto {}",
            loan.user.name,
            loan.instance.product.name,
            loan.instance.identifier,
            loan.date_start.format("%Y-%m-%d %H:%m"),
            loan.date_end.format("%Y-%m-%d %H:%m"),
        );
    }
}
