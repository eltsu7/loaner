-- Categories
INSERT INTO category (uuid, name)
VALUES ('9bda51a2-33f1-41b0-9caa-2c023f5d0c41', 'Cameras');
INSERT INTO category (uuid, name)
VALUES ('645bedf0-7616-4d7d-82f0-8b4e3ea58475', 'Lenses');

-- Products
INSERT INTO product (uuid, name, category)
VALUES ('96c8bbea-a187-4b9f-8f85-46d3ffbf3933', 'Canon EOS 5D Mark IV', '9bda51a2-33f1-41b0-9caa-2c023f5d0c41');
INSERT INTO product (uuid, name, category)
VALUES ('6af955c3-2a07-43ef-9565-1296fa503c5c', 'Canon EOS 6D Mark II', '9bda51a2-33f1-41b0-9caa-2c023f5d0c41');
INSERT INTO product (uuid, name, category)
VALUES ('2f34f003-d3cd-471e-9850-b55be1eb7117', 'Canon EF 24-70mm f/2.8L II USM', '645bedf0-7616-4d7d-82f0-8b4e3ea58475');
INSERT INTO product (uuid, name, category)
VALUES ('27287bfd-a830-4412-89ef-aa2b441a67eb', 'Canon EF 70-200mm f/2.8L IS II USM', '645bedf0-7616-4d7d-82f0-8b4e3ea58475');
INSERT INTO product (uuid, name, category)
VALUES ('32dfac9c-89a7-49ba-99fe-78c766b001f0', 'Canon EF 50mm f/1.8 STM', '645bedf0-7616-4d7d-82f0-8b4e3ea58475');

-- Instances
INSERT INTO instance (uuid, identifier, product)
VALUES ('b3b3b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#1', '96c8bbea-a187-4b9f-8f85-46d3ffbf3933');
INSERT INTO instance (uuid, identifier, product)
VALUES ('b4b4b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#2', '96c8bbea-a187-4b9f-8f85-46d3ffbf3933');
INSERT INTO instance (uuid, identifier, product)
VALUES ('b5b5b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#1', '6af955c3-2a07-43ef-9565-1296fa503c5c');
INSERT INTO instance (uuid, identifier, product)
VALUES ('b6b6b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#2', '6af955c3-2a07-43ef-9565-1296fa503c5c');
INSERT INTO instance (uuid, identifier, product)
VALUES ('b7b7b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#1', '2f34f003-d3cd-471e-9850-b55be1eb7117');
INSERT INTO instance (uuid, identifier, product)
VALUES ('b8b8b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#2', '2f34f003-d3cd-471e-9850-b55be1eb7117');
INSERT INTO instance (uuid, identifier, product)
VALUES ('b9b9b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#1', '27287bfd-a830-4412-89ef-aa2b441a67eb');
INSERT INTO instance (uuid, identifier, product)
VALUES ('c1c1c3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#2', '27287bfd-a830-4412-89ef-aa2b441a67eb');
INSERT INTO instance (uuid, identifier, product)
VALUES ('c2c2c3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#1', '32dfac9c-89a7-49ba-99fe-78c766b001f0');
INSERT INTO instance (uuid, identifier, product)
VALUES ('c3c3c3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '#2', '32dfac9c-89a7-49ba-99fe-78c766b001f0');

-- Users
INSERT INTO user (uuid, name)
VALUES ('38e3c8bb-52d9-40fb-bf66-e13f8f0e5bf6', 'Matti');
INSERT INTO user (uuid, name)
VALUES ('7f1f3ee7-9f6b-4cb0-a47c-2b21ae334fa1', 'Teppo');

-- Membership types
INSERT INTO membership_type (uuid, type)
VALUES ('f1b1b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', 'Basic');
INSERT INTO membership_type (uuid, type)
VALUES ('f2b2b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', 'Premium');

-- Membership payments
INSERT INTO membership_payments (uuid, user, membership_type, price, date_start, date_end)
VALUES ('b1b1b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '38e3c8bb-52d9-40fb-bf66-e13f8f0e5bf6', 'f1b1b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', 10.00, '2018-01-01', '2018-12-31');
INSERT INTO membership_payments (uuid, user, membership_type, price, date_start, date_end)
VALUES ('b2b2b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '7f1f3ee7-9f6b-4cb0-a47c-2b21ae334fa1', 'f2b2b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', 20.00, '2018-01-01', '2018-12-31');

-- Loans
INSERT INTO loan (uuid, user, date_start, date_end, accepted, description)
VALUES ('a1a1a3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '38e3c8bb-52d9-40fb-bf66-e13f8f0e5bf6', '2018-01-01', '2018-01-02', true, 'Test loan 1');
INSERT INTO loan (uuid, user, date_start, date_end, accepted, description)
VALUES ('a2a2a3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', '7f1f3ee7-9f6b-4cb0-a47c-2b21ae334fa1', '2018-01-01', '2018-01-02', true, 'Test loan 2');

-- Loan instances
INSERT INTO loan_instances (loan, instance)
VALUES ('a1a1a3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', 'b3b3b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b');
INSERT INTO loan_instances (loan, instance)
VALUES ('a1a1a3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', 'b5b5b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b');
INSERT INTO loan_instances (loan, instance)
VALUES ('a2a2a3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', 'b4b4b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b');
INSERT INTO loan_instances (loan, instance)
VALUES ('a2a2a3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b', 'b6b6b3b4-1b3b-4b1b-8b1b-1b3b1b3b1b3b');
