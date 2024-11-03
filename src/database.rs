use std::fs;
use uuid::Uuid;

pub use chrono::prelude::*;
use chrono_tz::Europe::Helsinki;
use chrono_tz::Tz;
use rusqlite::params;
use rusqlite::Connection;

#[derive(Default)]
pub struct LoanQueryParams {
    pub loan_uuid: Option<Uuid>,
    pub user_uuid: Option<Uuid>,
    pub product_uuid: Option<Uuid>,
    pub category_uuid: Option<Uuid>,
    pub date_start: Option<DateTime<Tz>>,
    pub date_end: Option<DateTime<Tz>>,
}

impl LoanQueryParams {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct Category {
    pub uuid: Uuid,
    pub name: String,
    pub supercategory: Option<Uuid>,
}

#[derive(Debug)]
pub struct Product {
    pub uuid: Uuid,
    pub name: String,
    pub category: Category,
}

#[derive(Debug)]
pub struct Instance {
    pub uuid: Uuid,
    pub identifier: String,
    pub product: Product,
}

#[derive(Debug)]
pub struct Loan {
    pub uuid: Uuid,
    pub instance: Instance,
    pub date_start: DateTime<Tz>,
    pub date_end: DateTime<Tz>,
    pub user: User,
}

pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn new(file_name: &str) -> Self {
        if file_name != "" {
            let db_exists = fs::metadata(file_name).is_ok();
            if db_exists {
                return Self {
                    connection: Connection::open(file_name).unwrap(),
                };
            } else {
                let db = Self {
                    connection: Connection::open(file_name).unwrap(),
                };
                db.initialize_database();
                return db;
            }
        } else {
            let db = Self {
                connection: Connection::open(file_name).unwrap(),
            };
            db.initialize_database();
            return db;
        }
    }

    fn initialize_database(&self) {
        let schema = include_str!("../schema.sql");
        self.connection.execute_batch(schema).unwrap();
    }

    pub fn get_users(&self) -> Vec<User> {
        let query = String::from(
            "SELECT
                user.uuid,
                user.name
            FROM user",
        );
        let mut statement = self.connection.prepare(&query).unwrap();
        let user_iter = statement
            .query_map([], |row| {
                Ok(User {
                    uuid: row.get(0).unwrap(),
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

    pub fn get_user(&self, uuid: Uuid) -> Option<User> {
        let query = String::from(
            "SELECT
                user.uuid,
                user.name
            FROM user
            WHERE user.uuid = ?1",
        );
        let mut statement = self.connection.prepare(&query).unwrap();
        let mut user_iter = statement
            .query_map(params![uuid], |row| {
                Ok(User {
                    uuid: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                })
            })
            .unwrap();
        if let Some(user) = user_iter.next() {
            return Some(user.unwrap());
        } else {
            None
        }
    }

    pub fn get_user_by_name(&self, name: &str) -> Option<User> {
        let query = String::from(
            "SELECT
                user.uuid,
                user.name
            FROM user
            WHERE user.name = ?1",
        );
        let mut statement = self.connection.prepare(&query).unwrap();
        let mut user_iter = statement
            .query_map(params![name], |row| {
                Ok(User {
                    uuid: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                })
            })
            .unwrap();
        if let Some(user) = user_iter.next() {
            return Some(user.unwrap());
        } else {
            None
        }
    }

    pub fn add_user(&self, name: &str) {
        let uuid = Uuid::new_v4();
        let query = String::from(
            "INSERT INTO
                user (uuid, name)
            VALUES
                (?1, ?2)",
        );
        self.connection
            .execute(&query, params![uuid, name])
            .unwrap();
    }

    pub fn get_categories(&self, supercategory: Option<Uuid>) -> Vec<Category> {
        let mut statement: rusqlite::Statement;
        if let Some(_) = supercategory {
            let query = String::from(
                "SELECT
                    category.uuid,
                    category.name,
                    category.supercategory
                FROM category
                WHERE category.supercategory = ?1",
            );
            statement = self.connection.prepare(&query).unwrap();
        } else {
            let query = String::from(
                "SELECT
                    category.uuid,
                    category.name,
                    category.supercategory
                FROM category",
            );
            statement = self.connection.prepare(&query).unwrap();
        }
        let category_iter = statement
            .query_map([], |row| {
                Ok(Category {
                    uuid: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    supercategory: row.get(2).unwrap(),
                })
            })
            .unwrap();

        let mut categories = Vec::new();

        for category in category_iter {
            categories.push(category.unwrap());
        }
        return categories;
    }

    pub fn get_category(&self, name: &str) -> Option<Category> {
        let query = String::from(
            "SELECT
                category.uuid,
                category.name,
                category.supercategory
            FROM category
            WHERE category.name = ?1",
        );
        let mut statement = self.connection.prepare(&query).unwrap();
        let mut category_iter = statement
            .query_map(params![name], |row| {
                Ok(Category {
                    uuid: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    supercategory: row.get(2).unwrap(),
                })
            })
            .unwrap();
        if let Some(category) = category_iter.next() {
            return Some(category.unwrap());
        } else {
            None
        }
    }

    pub fn add_category(
        &self,
        name: &str,
        supercategory: Option<Uuid>,
    ) -> Result<Category, String> {
        // Adding root category
        if supercategory.is_none() {
            let categories = self.get_categories(None);
            if categories.len() > 0 {
                return Err("Error: Supercategory must be specified.".to_string());
            } else {
                let uuid = Uuid::new_v4();
                self.connection
                    .execute(
                        "INSERT INTO category (uuid, name) VALUES (?1, ?2)",
                        params![uuid, name],
                    )
                    .unwrap();
                return Ok(self.get_category(name).unwrap());
            }
        }
        // Supercategory must exist
        let mut statement = self
            .connection
            .prepare("SELECT uuid FROM category WHERE uuid = ?1")
            .unwrap();
        let mut category_iter = statement
            .query_map(params![supercategory], |row| {
                Ok(row.get::<usize, Uuid>(0).unwrap())
            })
            .unwrap();
        if category_iter.next().is_none() {
            return Err("Error: Supercategory does not exist.".to_string());
        }

        // Category must not already exist
        let mut statement = self
            .connection
            .prepare("SELECT uuid FROM category WHERE name = ?1")
            .unwrap();
        let mut category_iter = statement
            .query_map(params![name], |row| Ok(row.get::<usize, Uuid>(0).unwrap()))
            .unwrap();
        if category_iter.next().is_some() {
            return Err("Error: Category already exists.".to_string());
        }

        let uuid = Uuid::new_v4();
        self.connection
            .execute(
                "INSERT INTO category (uuid, name, supercategory) VALUES (?1, ?2, ?3)",
                params![uuid, name, supercategory],
            )
            .unwrap();

        Ok(self.get_category(name).unwrap())
    }

    pub fn get_products(&self, category_id: Option<Uuid>) -> Vec<Product> {
        let mut query = String::from(
            "SELECT
                product.uuid,
                product.name,
                category.uuid,
                category.name,
                category.supercategory
            FROM product
            INNER JOIN category ON product.category = category.uuid",
        );
        if let Some(id) = category_id {
            query.push_str(&format!(" where category = {}", id));
        }

        let mut statement = self.connection.prepare(&query).unwrap();
        let product_iter = statement
            .query_map([], |row| {
                Ok(Product {
                    uuid: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    category: Category {
                        uuid: row.get(2).unwrap(),
                        name: row.get(3).unwrap(),
                        supercategory: row.get(4).unwrap(),
                    },
                })
            })
            .unwrap();

        let mut products = Vec::new();

        for product in product_iter {
            products.push(product.unwrap());
        }
        return products;
    }

    pub fn get_product_by_name(&self, name: &str) -> Option<Product> {
        let query = String::from(
            "SELECT
                product.uuid,
                product.name,
                category.uuid,
                category.name,
                category.supercategory
            FROM product
                INNER join category ON product.category = category.uuid
            WHERE product.name = ?1",
        );
        let mut statement = self.connection.prepare(&query).unwrap();
        let mut product_iter = statement
            .query_map(params![name], |row| {
                Ok(Product {
                    uuid: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    category: Category {
                        uuid: row.get(2).unwrap(),
                        name: row.get(3).unwrap(),
                        supercategory: row.get(4).unwrap(),
                    },
                })
            })
            .unwrap();

        // There should be only one product with the given name
        if let Some(product) = product_iter.next() {
            return Some(product.unwrap());
        } else {
            None
        }
    }

    pub fn get_product(&self, product_uuid: Uuid) -> Option<Product> {
        let query = String::from(
            "SELECT
                product.uuid,
                product.name,
                category.uuid,
                category.name,
                category.supercategory
            FROM product
                INNER JOIN category ON product.category = category.uuid
            WHERE product.uuid = ?1",
        );
        let mut statement = self.connection.prepare(&query).unwrap();
        let mut product_iter = statement
            .query_map(params![product_uuid], |row| {
                Ok(Product {
                    uuid: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    category: Category {
                        uuid: row.get(2).unwrap(),
                        name: row.get(3).unwrap(),
                        supercategory: row.get(4).unwrap(),
                    },
                })
            })
            .unwrap();
        if let Some(product) = product_iter.next() {
            return Some(product.unwrap());
        } else {
            None
        }
    }

    pub fn add_product(&self, name: &str, category_id: Uuid) -> Result<Product, String> {
        // Category must exist
        let mut statement = self
            .connection
            .prepare("SELECT uuid FROM category WHERE uuid = ?1")
            .unwrap();
        let mut category_iter = statement
            .query_map(params![category_id], |row| {
                Ok(row.get::<usize, Uuid>(0).unwrap())
            })
            .unwrap();
        if category_iter.next().is_none() {
            return Err("Error: Category does not exist.".to_string());
        }

        // Product must not already exist
        let mut statement = self
            .connection
            .prepare("SELECT uuid FROM product WHERE name = ?1")
            .unwrap();
        let mut product_iter = statement
            .query_map(params![name], |row| Ok(row.get::<usize, Uuid>(0).unwrap()))
            .unwrap();
        if product_iter.next().is_some() {
            return Err("Error: Product already exists.".to_string());
        }

        let uuid = Uuid::new_v4();
        self.connection
            .execute(
                "INSERT INTO product (uuid, name, category) VALUES (?1, ?2, ?3)",
                params![uuid, name, category_id],
            )
            .unwrap();

        return Ok(self.get_product(uuid).unwrap());
    }

    pub fn get_instances(&self, product_id: Option<Uuid>) -> Vec<Instance> {
        let mut instances = Vec::new();
        if product_id.is_some() {
            let query = String::from(
                "SELECT
                    instance.uuid,
                    instance.identifier,
                    product.uuid,
                    product.name,
                    category.uuid,
                    category.name,
                    category.supercategory
                FROM instance
                    INNER JOIN product ON instance.product = product.uuid
                    INNER JOIN category ON product.category = category.uuid
                WHERE product = ?1",
            );
            let mut statement = self.connection.prepare(&query).unwrap();
            let instance_iter = statement
                .query_map(params![product_id], |row| {
                    Ok(Instance {
                        uuid: row.get(0).unwrap(),
                        identifier: row.get(1).unwrap(),
                        product: Product {
                            uuid: row.get(2).unwrap(),
                            name: row.get(3).unwrap(),
                            category: Category {
                                uuid: row.get(4).unwrap(),
                                name: row.get(5).unwrap(),
                                supercategory: row.get(6).unwrap(),
                            },
                        },
                    })
                })
                .unwrap();
            for instance in instance_iter {
                instances.push(instance.unwrap());
            }
            return instances;
        } else {
            let query = String::from(
                "SELECt
                    instance.uuid,
                    instance.identifier,
                    product.uuid,
                    product.name,
                    category.uuid,
                    category.name,
                    category.supercategory
                FROM instance
                    INNER JOIN product ON instance.product = product.uuid
                    INNER JOIN category ON product.category = category.uuid",
            );
            let mut statement = self.connection.prepare(&query).unwrap();
            let instance_iter = statement
                .query_map([], |row| {
                    Ok(Instance {
                        uuid: row.get(0).unwrap(),
                        identifier: row.get(1).unwrap(),
                        product: Product {
                            uuid: row.get(2).unwrap(),
                            name: row.get(3).unwrap(),
                            category: Category {
                                uuid: row.get(4).unwrap(),
                                name: row.get(5).unwrap(),
                                supercategory: row.get(6).unwrap(),
                            },
                        },
                    })
                })
                .unwrap();
            for instance in instance_iter {
                instances.push(instance.unwrap());
            }
            return instances;
        }
    }

    pub fn get_instance(&self, instance_uuid: Uuid) -> Instance {
        let query = String::from(
            "SELECT
                instance.uuid,
                instance.identifier,
                product.uuid,
                product.name,
                category.uuid,
                category.name,
                category.supercategory
            FROM instance
                INNER JOIN product ON instance.product = product.uuid
                INNER JOIN category ON product.category = category.uuid
            WHERE instance.uuid = ?1",
        );
        self.connection
            .prepare(&query)
            .unwrap()
            .query_map(params![instance_uuid], |row| {
                Ok(Instance {
                    uuid: row.get(0).unwrap(),
                    identifier: row.get(1).unwrap(),
                    product: Product {
                        uuid: row.get(2).unwrap(),
                        name: row.get(3).unwrap(),
                        category: Category {
                            uuid: row.get(4).unwrap(),
                            name: row.get(5).unwrap(),
                            supercategory: row.get(6).unwrap(),
                        },
                    },
                })
            })
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
    }

    pub fn add_instance(&self, identifier: &str, product_uuid: Uuid) -> Result<Instance, String> {
        // Product must exist
        let mut statement = self
            .connection
            .prepare("SELECT uuid FROM product WHERE uuid = ?1")
            .unwrap();
        let mut product_iter = statement
            .query_map(params![product_uuid], |row| {
                Ok(row.get::<usize, Uuid>(0).unwrap())
            })
            .unwrap();
        if product_iter.next().is_none() {
            return Err("Error: Product does not exist.".to_string());
        }

        // Instance with this product id and identifier must not already exist
        let mut statement = self
            .connection
            .prepare("SELECT uuid FROM instance WHERE product = ?1 AND identifier = ?2")
            .unwrap();
        let mut instance_iter = statement
            .query_map(params![product_uuid, identifier], |row| {
                Ok(row.get::<usize, Uuid>(0).unwrap())
            })
            .unwrap();
        if instance_iter.next().is_some() {
            return Err("Error: Instance already exists.".to_string());
        }

        let uuid = Uuid::new_v4();
        self.connection
            .execute(
                "INSERT INTO instance (uuid, identifier, product) VALUES (?1, ?2, ?3)",
                params![uuid, identifier, product_uuid],
            )
            .unwrap();

        return Ok(self.get_instance(uuid));
    }

    /// Get loans from the database based on the query parameters
    /// Transform dates to Helsinki timezone
    pub fn get_loans(&self, params: LoanQueryParams) -> Vec<Loan> {
        let mut query = String::from(
            "SELECT 
                loan.uuid, 
                instance.uuid,
                instance.identifier,
                product.uuid,
                product.name,
                category.uuid,
                category.name,
                category.supercategory,
                date_start,
                date_end,
                user.uuid,
                user.name
            FROM loan 
                inner join user on loan.user = user.uuid
                inner join instance on loan.instance = instance.uuid
                inner join product on instance.product = product.uuid
                inner join category on product.category = category.uuid
            WHERE 1=1",
        );

        if let Some(id) = params.loan_uuid {
            query.push_str(&format!(" and loan.uuid = '{}'", id));
        }
        if let Some(id) = params.user_uuid {
            query.push_str(&format!(" and user.uuid = '{}'", id));
        }
        if let Some(id) = params.product_uuid {
            query.push_str(&format!(" and product.uuid = '{}'", id));
        }
        if let Some(id) = params.category_uuid {
            query.push_str(&format!(" and category.uuid = '{}'", id));
        }
        if let Some(start) = params.date_start {
            query.push_str(&format!(" and date_start >= '{}'", start));
        }
        if let Some(end) = params.date_end {
            query.push_str(&format!(" and date_end <= '{}'", end));
        }

        let mut statement = self.connection.prepare(&query).unwrap();
        let loan_iter = statement
            .query_map([], |row| {
                Ok(Loan {
                    uuid: row.get(0).unwrap(),
                    instance: Instance {
                        uuid: row.get(1).unwrap(),
                        identifier: row.get(2).unwrap(),
                        product: Product {
                            uuid: row.get(3).unwrap(),
                            name: row.get(4).unwrap(),
                            category: Category {
                                uuid: row.get(5).unwrap(),
                                name: row.get(6).unwrap(),
                                supercategory: row.get(7).unwrap(),
                            },
                        },
                    },
                    date_start: DateTime::parse_from_rfc3339(&row.get::<usize, String>(8).unwrap())
                        .unwrap()
                        .with_timezone(&Helsinki),
                    date_end: DateTime::parse_from_rfc3339(&row.get::<usize, String>(9).unwrap())
                        .unwrap()
                        .with_timezone(&Helsinki),
                    user: User {
                        uuid: row.get(10).unwrap(),
                        name: row.get(11).unwrap(),
                    },
                })
            })
            .unwrap();

        let mut loans = Vec::new();

        for loan in loan_iter {
            loans.push(loan.unwrap());
        }
        loans
    }

    pub fn get_loan(&self, loan_uuid: Uuid) -> Loan {
        let mut statement = self
            .connection
            .prepare(
                "SELECT 
                    loan.uuid, 
                    instance.uuid,
                    instance.identifier,
                    product.uuid,
                    product.name,
                    category.uuid,
                    category.name,
                    category.supercategory,
                    date_start,
                    date_end,
                    user.uuid,
                    user.name
                FROM loan 
                    inner join user on loan.user = user.uuid
                    inner join instance on loan.instance = instance.uuid
                    inner join product on instance.product = product.uuid
                    inner join category on product.category = category.uuid
                WHERE loan.uuid = ?1",
            )
            .unwrap();
        let mut loan_iter = statement
            .query_map(params![loan_uuid], |row| {
                Ok(Loan {
                    uuid: row.get(0).unwrap(),
                    instance: Instance {
                        uuid: row.get(1).unwrap(),
                        identifier: row.get(2).unwrap(),
                        product: Product {
                            uuid: row.get(3).unwrap(),
                            name: row.get(4).unwrap(),
                            category: Category {
                                uuid: row.get(5).unwrap(),
                                name: row.get(6).unwrap(),
                                supercategory: row.get(7).unwrap(),
                            },
                        },
                    },
                    date_start: DateTime::parse_from_rfc3339(&row.get::<usize, String>(8).unwrap())
                        .unwrap()
                        .with_timezone(&Helsinki),
                    date_end: DateTime::parse_from_rfc3339(&row.get::<usize, String>(9).unwrap())
                        .unwrap()
                        .with_timezone(&Helsinki),
                    user: User {
                        uuid: row.get(10).unwrap(),
                        name: row.get(11).unwrap(),
                    },
                })
            })
            .unwrap();
        loan_iter.next().unwrap().unwrap()
    }

    pub fn add_loan(
        &self,
        user_id: Uuid,
        instance_id: Uuid,
        date_start: DateTime<Tz>,
        date_end: DateTime<Tz>,
    ) -> Result<Loan, String> {
        // Check overalapping loans
        let mut statement = self
            .connection
            .prepare(
                "SELECT 
                    user, 
                    date_start, 
                    date_end 
                FROM 
                    loan 
                WHERE 
                    instance = ?1 
                    AND (
                        (date_start <= ?2 AND date_end >= ?2) 
                        OR (date_start <= ?3 AND date_end >= ?3) 
                        OR (date_start >= ?2 AND date_end <= ?3)
                    )",
            )
            .map_err(|e| e.to_string())?;

        let mut conflicting_loans = statement
            .query_map(
                params![instance_id, date_start.to_rfc3339(), date_end.to_rfc3339(),],
                |row| {
                    Ok((
                        row.get::<_, Uuid>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .map_err(|e| e.to_string())?;

        if let Some(conflicting_loan) = conflicting_loans.next() {
            let (conflicting_user_id, conflicting_date_start, conflicting_date_end) =
                conflicting_loan.map_err(|e| e.to_string())?;
            let error_message = format!(
                "Error: Instance is already loaned in the requested time frame.\n\
                 Conflicting Loan - User ID: {}, Date Start: {}, Date End: {}",
                conflicting_user_id, conflicting_date_start, conflicting_date_end,
            );
            return Err(error_message);
        }

        // Insert the new loan if no conflicts
        let uuid = Uuid::new_v4();
        self.connection
            .execute(
                "INSERT INTO
                    loan (uuid, user, instance, date_start, date_end)
                VALUES
                    (?1, ?2, ?3, ?4, ?5)",
                params![
                    uuid,
                    user_id,
                    instance_id,
                    date_start.to_rfc3339(),
                    date_end.to_rfc3339(),
                ],
            )
            .map_err(|e| e.to_string())?;
        return Ok(self.get_loan(uuid));
    }
}
