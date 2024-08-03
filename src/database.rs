use rusqlite::Connection;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub supercategory: i32, // How does one handle recursion
}

#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub category: Category,
}

#[derive(Debug)]
pub struct Instance {
    pub id: i32,
    pub identifier: String,
    pub product: Product,
}

#[derive(Debug)]
pub struct Loan {
    pub id: i32,
    pub instance: Instance,
    pub date_start: String,
    pub date_end: String,
    pub user: User,
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(file_name: &str) -> Self {
        Self {
            connection: Connection::open(file_name).unwrap(),
        }
    }

    pub fn get_users(&self) -> Vec<User> {
        let mut statement = self.connection.prepare("select * from user;").unwrap();
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

    pub fn get_loans(&self) -> Vec<Loan> {
        let mut statement = self
            .connection
            .prepare(
                "select 
                    loan.id, 
                    instance.id,
                    instance.identifier,
                    product.id,
                    product.name,
                    category.id,
                    category.name,
                    category.supercategory,
                    date_start,
                    date_end,
                    user.id,
                    user.name
                from loan 
                    inner join user on loan.user = user.id
                    inner join instance on loan.instance = instance.id
                    inner join product on instance.product = product.id
                    inner join category on product.category = category.id
            ;",
            )
            .unwrap();
        let loan_iter = statement
            .query_map([], |row| {
                Ok(Loan {
                    id: row.get(0).unwrap(),
                    instance: Instance {
                        id: row.get(1).unwrap(),
                        identifier: row.get(2).unwrap(),
                        product: Product {
                            id: row.get(3).unwrap(),
                            name: row.get(4).unwrap(),
                            category: Category {
                                id: row.get(5).unwrap(),
                                name: row.get(6).unwrap(),
                                supercategory: row.get(7).unwrap(),
                            },
                        },
                    },
                    date_start: row.get(8).unwrap(),
                    date_end: row.get(9).unwrap(),
                    user: User {
                        id: row.get(10).unwrap(),
                        name: row.get(11).unwrap(),
                    },
                })
            })
            .unwrap();

        let mut loans = Vec::new();

        for user in loan_iter {
            loans.push(user.unwrap());
        }
        return loans;
    }
}
