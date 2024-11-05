CREATE TABLE IF NOT EXISTS category (
  uuid blob NOT NULL PRIMARY KEY,
  name text NOT NULL,
  supercategory blob
);


CREATE TABLE IF NOT EXISTS instance (
  uuid blob NOT NULL PRIMARY KEY,
  identifier text NOT NULL,
  product blob NOT NULL,
  FOREIGN KEY (product) REFERENCES product (uuid)
);


CREATE TABLE IF NOT EXISTS loan (
  uuid blob NOT NULL PRIMARY KEY,
  user blob NOT NULL,
  date_start text NOT NULL,
  date_end text NOT NULL,
  accepted boolean NOT NULL,
  description text,
  FOREIGN KEY (user) REFERENCES user (uuid)
);


CREATE TABLE IF NOT EXISTS product (
  uuid blob NOT NULL PRIMARY KEY,
  name text NOT NULL,
  category blob NOT NULL,
  FOREIGN KEY (category) REFERENCES category (uuid)
);


CREATE TABLE IF NOT EXISTS user (
  uuid blob NOT NULL PRIMARY KEY,
  name text NOT NULL
);


CREATE TABLE IF NOT EXISTS membership_payments (
  uuid blob NOT NULL PRIMARY KEY,
  user blob NOT NULL,
  membership_type blob NOT NULL,
  price numeric NOT NULL,
  date_start text NOT NULL,
  date_end text NOT NULL,
  FOREIGN KEY (user) REFERENCES user (uuid),
  FOREIGN KEY (membership_type) REFERENCES membership_type (uuid)
);

CREATE TABLE IF NOT EXISTS membership_type (
  uuid blob NOT NULL PRIMARY KEY,
  type text NOT NULL
);

CREATE TABLE IF NOT EXISTS loan_instances (
  loan blob NOT NULL,
  instance blob NOT NULL,
  PRIMARY KEY (loan, instance),
  FOREIGN KEY (loan) REFERENCES loan (uuid),
  FOREIGN KEY (instance) REFERENCES instance (uuid)
);
