#[allow(dead_code)]
pub fn initialize_test_database(db_name: Option<&str>) -> crate::database::Database {
    use crate::database::Database;

    let db_name = db_name.unwrap_or("");
    let db = Database::new(&db_name);

    let user_names = vec!["Alice", "Bob", "Charlie"];
    for user_name in &user_names {
        let _ = db.add_user(user_name);
    }

    let catalogue = db.add_category("Catalogue", None).unwrap();

    let cameras = db.add_category("Cameras", Some(catalogue.uuid)).unwrap();

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
    let _ = db.add_instance("#2", canon_zoom_2.uuid);

    db
}

#[test]
fn test_initialization() {
    let _db = initialize_test_database(None);
}

#[test]
fn test_users() {
    let db = initialize_test_database(None);

    let user_count = db.get_users().len();

    assert_eq!(user_count, 3);

    let new_user_names = vec!["David", "Eve", "Frank"];

    for user_name in &new_user_names {
        let _ = db.add_user(user_name);
    }

    let users = db.get_users();
    assert_eq!(users.len(), 6);
    println!("User count: {}", users.len());
    for user in &users {
        println!("User: {} {}", user.uuid, user.name);
    }

    assert!(users[0].name == "Alice");
    assert!(users[1].name == "Bob");
    assert!(users[2].name == "Charlie");
    assert!(users[3].name == "David");
    assert!(users[4].name == "Eve");
    assert!(users[5].name == "Frank");
}

#[test]
fn test_categories() {
    let db = initialize_test_database(None);

    let mut categories = db.get_categories(None);
    for category in &categories {
        println!("Category: {}", category.name);
    }
    assert_eq!(categories.len(), 3);
    assert!(categories[0].name == "Catalogue");
    assert!(categories[1].name == "Cameras");
    assert!(categories[2].name == "Lenses");

    let new_category_names = vec!["Drones", "Flashes", "Gimbals"];

    let catalogue_uuid = db.get_category("Catalogue").unwrap().uuid;

    for category_name in &new_category_names {
        let result = db.add_category(category_name, Some(catalogue_uuid));
        assert!(!result.is_err());
    }

    categories = db.get_categories(None);
    assert_eq!(categories.len(), 6);

    assert!(categories[3].name == "Drones");
    assert!(categories[4].name == "Flashes");
    assert!(categories[5].name == "Gimbals");

    let drones_uuid = db.get_category("Drones").unwrap().uuid;
    let mavic = db.add_product("Mavic 2 Pro", drones_uuid).unwrap();

    let result = db.remove_category(drones_uuid);
    assert!(result.is_err());

    let result = db.remove_product(mavic.uuid);
    assert!(result.is_ok());

    let result = db.remove_category(drones_uuid);
    assert!(result.is_ok());
}

#[test]
fn test_duplicate_categories() {
    let db = initialize_test_database(None);

    let new_category_names = vec!["Cameras", "Lenses"];

    let catalogue_uuid = db.get_category("Catalogue").unwrap().uuid;

    for category_name in &new_category_names {
        let result = db.add_category(category_name, Some(catalogue_uuid));
        assert!(result.is_err());
    }

    let categories = db.get_categories(None);
    assert_eq!(categories.len(), 3);

    let lenses_uuid = db.get_category("Lenses").unwrap().uuid;

    assert!(db.add_category("RF-S", Some(lenses_uuid)).is_ok());
    assert!(db.add_category("RF-S", Some(lenses_uuid)).is_err());
}

#[test]
fn test_add_loan() {
    let db = initialize_test_database(None);

    let user = &db.get_users()[0];
    let product = &db.get_product_by_name("Canon R6").unwrap();
    let instance = &db.get_instances(Some(product.uuid))[0];

    let now = chrono::Utc::now().with_timezone(&chrono_tz::Europe::Helsinki);

    let loan = db.add_loan(
        user.uuid,
        vec![instance.uuid],
        now,
        now + chrono::Duration::days(7),
    );
    assert!(loan.is_ok());
}

#[test]
fn test_add_overlapping_loan() {
    let db = initialize_test_database(None);

    let user = &db.get_users()[0];
    let product = &db.get_product_by_name("Canon R6").unwrap();
    let instance = &db.get_instances(Some(product.uuid))[0];

    let now = chrono::Utc::now().with_timezone(&chrono_tz::Europe::Helsinki);

    let loan = db.add_loan(
        user.uuid,
        vec![instance.uuid],
        now,
        now + chrono::Duration::days(7),
    );
    assert!(loan.is_ok());

    let overlapping_loan = db.add_loan(
        user.uuid,
        vec![instance.uuid],
        now + chrono::Duration::days(1),
        now + chrono::Duration::days(8),
    );
    assert!(overlapping_loan.is_err());
}

#[test]
fn test_loan_filters() {
    let db = initialize_test_database(None);

    let user_1 = &db.get_users()[0];
    let user_2 = &db.get_users()[1];
    let product_1 = &db.get_product_by_name("Canon R6").unwrap();
    let product_2 = &db.get_product_by_name("Hasselblad 500c").unwrap();
    let instance_1 = &db.get_instances(Some(product_1.uuid))[0];
    let instance_2 = &db.get_instances(Some(product_2.uuid))[0];

    let now = chrono::Utc::now().with_timezone(&chrono_tz::Europe::Helsinki);

    let loan_1 = db.add_loan(
        user_1.uuid,
        vec![instance_1.uuid],
        now,
        now + chrono::Duration::days(7),
    );
    assert!(loan_1.is_ok());
    let loan_2 = db.add_loan(
        user_2.uuid,
        vec![instance_2.uuid],
        now,
        now + chrono::Duration::days(7),
    );
    assert!(loan_2.is_ok());

    let loans = db.get_loans(crate::database::LoanQueryParams::new());
    assert!(loans.len() == 2);

    let user_1_loans = db.get_loans(crate::database::LoanQueryParams {
        user_uuid: Some(user_1.uuid),
        ..Default::default()
    });
    assert!(user_1_loans.len() == 1);
    assert!(user_1_loans[0].user.uuid == user_1.uuid);

    let user_2_loans = db.get_loans(crate::database::LoanQueryParams {
        user_uuid: Some(user_2.uuid),
        ..Default::default()
    });
    assert!(user_2_loans.len() == 1);
    assert!(user_2_loans[0].user.uuid == user_2.uuid);

    let product_1_loans = db.get_loans(crate::database::LoanQueryParams {
        product_uuid: Some(product_1.uuid),
        ..Default::default()
    });
    assert!(product_1_loans.len() == 1);
    assert!(product_1_loans[0].instaces[0].product.uuid == product_1.uuid);

    let product_2_loans = db.get_loans(crate::database::LoanQueryParams {
        product_uuid: Some(product_2.uuid),
        ..Default::default()
    });
    assert!(product_2_loans.len() == 1);
    assert!(product_2_loans[0].instaces[0].product.uuid == product_2.uuid);
}

#[test]
fn test_loans() {
    use chrono_tz::Europe::Helsinki;
    let db = initialize_test_database(None);

    let loan_count = db.get_loans(crate::database::LoanQueryParams::new()).len();
    assert_eq!(loan_count, 0);

    let user = &db.get_users()[0];
    let product = &db.get_product_by_name("Canon R6").unwrap();
    let instance = &db.get_instances(Some(product.uuid))[0];

    let now = chrono::Utc::now().with_timezone(&Helsinki);

    let loan = db.add_loan(
        user.uuid,
        vec![instance.uuid],
        now,
        now + chrono::Duration::days(7),
    );
    dbg!(&loan);
    assert!(loan.is_ok());

    let overlapping_loan_1 = db.add_loan(
        user.uuid,
        vec![instance.uuid],
        now + chrono::Duration::days(1),
        now + chrono::Duration::days(8),
    );
    assert!(overlapping_loan_1.is_err());

    let overlapping_loan_2 = db.add_loan(
        user.uuid,
        vec![instance.uuid],
        now - chrono::Duration::days(1),
        now + chrono::Duration::days(6),
    );
    assert!(overlapping_loan_2.is_err());

    let overlapping_loan_3 = db.add_loan(
        user.uuid,
        vec![instance.uuid],
        now - chrono::Duration::days(7),
        now + chrono::Duration::days(1),
    );
    assert!(overlapping_loan_3.is_err());

    assert!(db.get_loans(crate::database::LoanQueryParams::new()).len() == 1);
}
