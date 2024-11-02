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
INSERT INTO loan VALUES(4,1,1,'2024-12-09T13:20:56.571537','2024-12-12T13:20:56.571537');
INSERT INTO loan VALUES(5,1,1,'2024-02-01T13:20:56.571537','2024-02-05T13:20:56.571537');
INSERT INTO loan VALUES(6,1,1,'2024-05-29T13:20:56.571537','2024-06-21T13:20:56.571537');
INSERT INTO loan VALUES(7,1,2,'2024-08-16T13:20:56.571537','2024-08-24T13:20:56.571537');
INSERT INTO loan VALUES(8,2,1,'2024-07-27T13:20:56.571537','2024-08-07T13:20:56.571537');
INSERT INTO loan VALUES(9,1,1,'2024-03-04T13:20:56.571537','2024-03-07T13:20:56.571537');
INSERT INTO loan VALUES(10,1,2,'2025-03-08T13:20:56.571537','2025-03-21T13:20:56.571537');
INSERT INTO loan VALUES(11,1,1,'2024-09-17T13:20:56.571537','2024-09-18T13:20:56.571537');
INSERT INTO loan VALUES(12,2,2,'2025-10-12T13:20:56.571537','2025-10-12T13:20:56.571537');
INSERT INTO loan VALUES(13,2,1,'2024-11-02T13:20:56.571537','2024-11-05T13:20:56.571537');
INSERT INTO loan VALUES(14,2,2,'2024-10-01T13:20:56.571537','2024-10-15T13:20:56.571537');
INSERT INTO loan VALUES(15,1,1,'2024-06-11T13:20:56.571537','2024-06-20T13:20:56.571537');
INSERT INTO loan VALUES(16,2,2,'2024-04-01T13:20:56.571537','2024-04-10T13:20:56.571537');
INSERT INTO loan VALUES(17,2,1,'2024-05-15T13:20:56.571537','2024-05-30T13:20:56.571537');
INSERT INTO loan VALUES(18,1,2,'2024-07-01T13:20:56.571537','2024-07-15T13:20:56.571537');
INSERT INTO loan VALUES(19,1,1,'2024-08-20T13:20:56.571537','2024-08-25T13:20:56.571537');
INSERT INTO loan VALUES(20,2,2,'2024-09-01T13:20:56.571537','2024-09-10T13:20:56.571537');
INSERT INTO loan VALUES(21,1,1,'2024-10-05T13:20:56.571537','2024-10-15T13:20:56.571537');
INSERT INTO loan VALUES(22,2,1,'2024-11-10T13:20:56.571537','2024-11-20T13:20:56.571537');
INSERT INTO loan VALUES(23,1,2,'2025-01-01T13:20:56.571537','2025-01-10T13:20:56.571537');
INSERT INTO loan VALUES(24,2,1,'2024-09-10T13:20:56.571537','2024-09-29T13:20:56.571537');
INSERT INTO loan VALUES(25,1,2,'2025-04-29T13:20:56.571537','2025-05-15T13:20:56.571537');
INSERT INTO loan VALUES(26,2,1,'2024-12-08T13:20:56.571537','2025-01-01T13:20:56.571537');
INSERT INTO loan VALUES(27,2,1,'2025-03-03T13:20:56.571537','2025-03-13T13:20:56.571537');
INSERT INTO loan VALUES(28,2,2,'2025-04-01T13:20:56.571537','2025-04-05T13:20:56.571537');
INSERT INTO loan VALUES(29,2,1,'2024-12-17T13:20:56.571537','2025-01-12T13:20:56.571537');
INSERT INTO loan VALUES(30,1,2,'2024-04-12T13:20:56.571537','2024-05-06T13:20:56.571537');
INSERT INTO loan VALUES(31,2,2,'2024-01-16T13:20:56.571537','2024-01-25T13:20:56.571537');
INSERT INTO loan VALUES(32,2,2,'2023-12-28T13:20:56.571537','2024-01-06T13:20:56.571537');
INSERT INTO loan VALUES(33,1,1,'2024-09-09T13:20:56.571537','2024-09-29T13:20:56.571537');
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('user',2);
INSERT INTO sqlite_sequence VALUES('category',9);
INSERT INTO sqlite_sequence VALUES('product',8);
INSERT INTO sqlite_sequence VALUES('loan',33);
INSERT INTO sqlite_sequence VALUES('instance',2);
COMMIT;
