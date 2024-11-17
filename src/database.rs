use rusqlite::params_from_iter;
use std::fs;
use uuid::Uuid;

pub use chrono::prelude::*;
use chrono_tz::Europe::Helsinki;
use chrono_tz::Tz;
use rusqlite::params;
use rusqlite::Connection;

#[derive(Default, Debug, Clone)]
pub struct LoanQueryParams {
    pub loan_uuid: Option<Uuid>,
    pub loan_accepted: Option<bool>,
    pub user_uuid: Option<Uuid>,
    pub product_uuid: Option<Uuid>,
    pub instance_uuid: Option<Uuid>,
    pub category_uuid: Option<Uuid>,
    pub date_start: Option<DateTime<Tz>>,
    pub date_end: Option<DateTime<Tz>>,
}

impl LoanQueryParams {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub uuid: Uuid,
    pub name: String,
    pub supercategory: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct Product {
    pub uuid: Uuid,
    pub name: String,
    pub category: Category,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub uuid: Uuid,
    pub identifier: String,
    pub product: Product,
}

#[derive(Debug, Clone)]
pub struct Loan {
    pub uuid: Uuid,
    pub user: User,
    pub date_start: DateTime<Tz>,
    pub date_end: DateTime<Tz>,
    pub accepted: bool,
    pub description: Option<String>,
    pub instaces: Vec<Instance>,
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

    pub fn add_user(&self, name: &str) -> Result<User, String> {
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

        match self.get_user_by_name(name) {
            Some(user) => Ok(user),
            None => Err("Error: User not found.".to_string()),
        }
    }

    pub fn remove_user(&self, uuid: Uuid) -> Result<(), String> {
        let query = String::from(
            "DELETE FROM user
            WHERE user.uuid = ?1",
        );
        self.connection
            .execute(&query, params![uuid])
            .map_err(|e| e.to_string())?;
        Ok(())
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

    pub fn remove_category(&self, uuid: Uuid) -> Result<(), String> {
        let query = String::from(
            "DELETE FROM category
            WHERE category.uuid = ?1",
        );
        self.connection
            .execute(&query, params![uuid])
            .map_err(|e| e.to_string())?;
        Ok(())
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

    /// Get loans from loan_view
    /// Transform dates to Helsinki timezone
    pub fn get_loans(&self, params: LoanQueryParams) -> Vec<Loan> {
        let mut query = String::from(
            "SELECT 
                loan_uuid,
                loan_date_start,
                loan_date_end,
                loan_accepted,
                loan_description,
                user_uuid,
                user_name,
                instance_uuid,
                instance_identifier,
                product_uuid,
                product_name,
                category_uuid,
                category_name,
                category_supercategory
            FROM loan_view
            WHERE 1=1",
        );

        let mut query_params: Vec<&(dyn rusqlite::ToSql + Sync)> = Vec::new();
        let mut date_strings: Vec<String> = Vec::new();

        if let Some(ref id) = params.loan_uuid {
            query.push_str(" AND loan_uuid = ?");
            query_params.push(id);
        }
        if let Some(ref id) = params.instance_uuid {
            query.push_str(" AND instance_uuid = ?");
            query_params.push(id);
        }
        if let Some(ref accepted) = params.loan_accepted {
            query.push_str(" AND loan_accepted = ?");
            query_params.push(accepted);
        }
        if let Some(ref id) = params.user_uuid {
            query.push_str(" AND user_uuid = ?");
            query_params.push(id);
        }
        if let Some(ref id) = params.product_uuid {
            query.push_str(" AND product_uuid = ?");
            query_params.push(id);
        }
        if let Some(ref id) = params.category_uuid {
            query.push_str(" AND category_uuid = ?");
            query_params.push(id);
        }
        if let Some(ref start) = params.date_start {
            date_strings.push(start.to_rfc3339());
            query.push_str(" AND loan_date_end >= ?");
        }
        if let Some(ref end) = params.date_end {
            date_strings.push(end.to_rfc3339());
            query.push_str(" AND loan_date_start <= ?");
        }

        for date in date_strings.iter() {
            query_params.push(date);
        }

        // Combine loans
        let mut loans: Vec<Loan> = Vec::new();

        let mut statement = self.connection.prepare(&query).unwrap();
        let rows = statement.query_map(params_from_iter(query_params.iter()), |row| {
            Ok(Loan {
                uuid: row.get(0).unwrap(),
                user: User {
                    uuid: row.get(5).unwrap(),
                    name: row.get(6).unwrap(),
                },
                date_start: DateTime::parse_from_rfc3339(
                    &row.get::<usize, String>(1).unwrap().as_str(),
                )
                .unwrap()
                .with_timezone(&Helsinki),
                date_end: DateTime::parse_from_rfc3339(
                    &row.get::<usize, String>(2).unwrap().as_str(),
                )
                .unwrap()
                .with_timezone(&Helsinki),

                accepted: row.get(3).unwrap(),
                description: row.get(4).unwrap(),
                instaces: vec![Instance {
                    uuid: row.get(7).unwrap(),
                    identifier: row.get(8).unwrap(),
                    product: Product {
                        uuid: row.get(9).unwrap(),
                        name: row.get(10).unwrap(),
                        category: Category {
                            uuid: row.get(11).unwrap(),
                            name: row.get(12).unwrap(),
                            supercategory: row.get(13).unwrap(),
                        },
                    },
                }],
            })
        });

        // Fill instances for each loan
        'rows: for row in rows.unwrap() {
            for loan in loans.iter_mut() {
                if loan.uuid == row.as_ref().unwrap().uuid {
                    let instance = row.as_ref().unwrap().instaces[0].clone();
                    loan.instaces.push(instance);
                    continue 'rows;
                }
            }
            loans.push(row.unwrap());
        }

        loans
    }

    pub fn get_loan(&self, loan_uuid: Uuid) -> Option<Loan> {
        let query_params = LoanQueryParams {
            loan_uuid: Some(loan_uuid),
            ..Default::default()
        };
        let loans = self.get_loans(query_params);

        if !loans.is_empty() {
            return Some(loans[0].clone());
        } else {
            None
        }
    }

    pub fn add_loan(
        &self,
        user_id: Uuid,
        instaces: Vec<Uuid>,
        date_start: DateTime<Tz>,
        date_end: DateTime<Tz>,
    ) -> Result<Loan, String> {
        // Check overalapping loans

        for instance_id in instaces.iter() {
            let query_params = LoanQueryParams {
                instance_uuid: Some(*instance_id),
                date_start: Some(date_start),
                date_end: Some(date_end),
                loan_accepted: Some(true),
                ..Default::default()
            };

            let loans = self.get_loans(query_params);

            dbg!(&loans);

            if loans.len() > 0 {
                let error_message = format!(
                    "Error: Instance is already loaned in the requested time frame.\n\
                 Conflicting Loan - User ID: {}, Date Start: {}, Date End: {}",
                    loans[0].user.uuid, loans[0].date_start, loans[0].date_end
                );
                return Err(error_message);
            }
        }

        // Insert the new loan if no conflicts
        let loan_uuid = Uuid::new_v4();

        // Manual acceptance for loans longer than 7 days
        let mut accepted = true;
        if date_end - date_start > chrono::Duration::days(7) {
            accepted = false;
        }

        let add_loan_query = String::from(
            "INSERT INTO
                loan (uuid, user, date_start, date_end, accepted, description)
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6)",
        );
        self.connection
            .execute(
                &add_loan_query,
                params![
                    loan_uuid,
                    user_id,
                    date_start.to_rfc3339(),
                    date_end.to_rfc3339(),
                    accepted,
                    None::<String>,
                ],
            )
            .map_err(|e| e.to_string())?;

        let add_loan_instance_query = String::from(
            "INSERT INTO
                loan_instances (loan, instance)
            VALUES
                (?1, ?2)",
        );

        for instance_id in instaces {
            let result = self
                .connection
                .execute(&add_loan_instance_query, params![loan_uuid, instance_id])
                .map_err(|e| e.to_string());
            assert!(result.is_ok());
        }

        let new_loan = self.get_loan(loan_uuid);
        match new_loan {
            Some(loan) => {
                return Ok(loan);
            }
            None => {
                return Err("Error: Loan not found.".to_string());
            }
        }
    }
}
