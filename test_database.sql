PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE user(id integer primary key autoincrement, name text not null);
INSERT INTO user VALUES(1,'Eeli');
INSERT INTO user VALUES(2,'Daniel');
CREATE TABLE category(id integer primary key autoincrement, name text not null, supercategory integer, foreign key (supercategory) references category(id));
INSERT INTO category VALUES(1,'Catalogue',NULL);
INSERT INTO category VALUES(2,'Cameras',1);
INSERT INTO category VALUES(3,'Lights',1);
INSERT INTO category VALUES(4,'Digital Cameras',2);
INSERT INTO category VALUES(5,'Film Cameras',2);
INSERT INTO category VALUES(6,'Strobes',3);
INSERT INTO category VALUES(7,'Video Lights',3);
INSERT INTO category VALUES(8,'Accessories',1);
INSERT INTO category VALUES(9,'Camera Straps',8);
CREATE TABLE product (id integer primary key autoincrement, name text, category integer not null, foreign key (category) references category(id));
INSERT INTO product VALUES(1,'EOS R6',4);
INSERT INTO product VALUES(2,'EOS 6D',4);
INSERT INTO product VALUES(3,'EOS 200D',4);
INSERT INTO product VALUES(4,'EOS 5',5);
INSERT INTO product VALUES(5,'Hasselblad 500C',5);
INSERT INTO product VALUES(6,'Godox AD200',6);
INSERT INTO product VALUES(7,'LED Panel 3000',7);
INSERT INTO product VALUES(8,'Canon strap',9);
CREATE TABLE instance (id integer primary key autoincrement, identifier text, product integer not null, foreign key (product) references product(id));
INSERT INTO instance VALUES(1,'#1',1);
INSERT INTO instance VALUES(2,'Kultainen',5);
CREATE TABLE loan (id integer primary key autoincrement, user integer not null, instance integer not null, date_start text not null, date_end text not null, foreign key (user) references user(id), foreign key (instance) references instance(id));
INSERT INTO loan VALUES(1,1,1,'2024-08-04T14:48:04.129047500+03:00','2024-08-06T14:48:04.129047500+03:00');
INSERT INTO loan VALUES(3,1,2,'2024-08-04T20:55:49.591374700+03:00','2024-08-04T20:55:49.591917+03:00');
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('user',2);
INSERT INTO sqlite_sequence VALUES('category',9);
INSERT INTO sqlite_sequence VALUES('product',8);
INSERT INTO sqlite_sequence VALUES('loan',3);
INSERT INTO sqlite_sequence VALUES('instance',2);
COMMIT;
