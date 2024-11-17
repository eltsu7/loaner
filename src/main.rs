use database::LoanQueryParams;

pub mod database;
pub mod test_database;

fn add_test_data(db: &database::Database) {
    let catalogue = db.add_category("Catalogue", None).unwrap();

    let cameras = db.add_category("Cameras", Some(catalogue.uuid)).unwrap();

    let user_1 = db.add_user("Alice").unwrap();

    let canon_r6 = db.add_product("Canon R6", cameras.uuid).unwrap();
    let _ = db.add_instance("#1", canon_r6.uuid);
    let _ = db.add_instance("#2", canon_r6.uuid);

    let hassel = db.add_product("Hasselblad 500c", cameras.uuid).unwrap();
    let _ = db.add_instance("#1", hassel.uuid);
    let _ = db.add_instance("#2", hassel.uuid);

    let lenses = db.add_category("Lenses", Some(catalogue.uuid)).unwrap();

    let canon_zoom_1 = db.add_product("Canon 24-70mm f/2.8", lenses.uuid).unwrap();
    let _ = db.add_instance("#1", canon_zoom_1.uuid);
    let _ = db.add_instance("#2", canon_zoom_1.uuid);
    let canon_zoom_2 = db.add_product("Canon 70-200mm f/2.8", lenses.uuid).unwrap();
    let _ = db.add_instance("#1", canon_zoom_2.uuid);
    let ins_2 = db.add_instance("#2", canon_zoom_2.uuid).unwrap();

    let now = chrono::Utc::now().with_timezone(&chrono_tz::Europe::Helsinki);

    let _loan = db
        .add_loan(
            user_1.uuid,
            vec![ins_2.uuid],
            now,
            now + chrono::Duration::days(7),
        )
        .unwrap();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <database>", args[0]);
        std::process::exit(1);
    }
    let db = database::Database::new(&args[1]);

    add_test_data(&db);

    let user_count = &db.get_users().len();
    let product_count = &db.get_products(None).len();
    let category_count = &db.get_categories(None).len();
    let loan_count = &db.get_loans(LoanQueryParams::new()).len();

    println!(
        "Database connected, {} users, {} products, {} categories, {} loans",
        user_count, product_count, category_count, loan_count
    );
}
