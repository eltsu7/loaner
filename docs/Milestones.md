# v0.0.1
In scope:
- Program compiles :)
- Choose database and technologies
- Rudimentary database schema
- User can create very rudimentary loan
Out of scope:
- User handling
- Catalogue handling

### Notes:
DB options: some sql
Rust + sqlx? Diesel?

Test database:
- Catalogue
	- Cameras
		- Digital
			- Eos R6
			- Eos 6D
			- Eos 200D
		- Film
			- Eos 5
			- Hasselblad 500c
	- Lights
		- Strobes
			- Godox AD200
		- Video light
			- LED Panel 3000
	- Accessories
		- Camera straps
			- Strap 1

Tables needed in first version of the database:
- Category
	- (id)
	- Name
	- supercategory -> category.id (nullable)
- Product
	- (id)
	- Name
	- category -> category.id
- Instance (name wip)
	- (id)
	- Identifier (add-on to product name)
	- product -> product.id
- User
	- (id)
	- Name
- Loan
	- (id)
	- user -> user.id
	- instance -> instance.id
	- Date start
	- Date end
