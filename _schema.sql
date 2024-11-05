PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE user(uuid blob primary key not null, name text not null);
CREATE TABLE category(uuid blob primary key not null, name text not null, supercategory blob, foreign key (supercategory) references category(id));
CREATE TABLE product (uuid blob primary key not null, name text, category blob not null, foreign key (category) references category(id));
CREATE TABLE instance (uuid blob primary key not null, identifier text, product blob not null, foreign key (product) references product(id));
CREATE TABLE loan (uuid blob primary key not null, user blob not null, instance blob not null, date_start text not null, date_end text not null, foreign key (user) references user(id), foreign key (instance) references instance(id));
COMMIT;
