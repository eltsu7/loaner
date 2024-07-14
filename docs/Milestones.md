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
	- Name
	- Parent category (nullable)
- Product
	- Name
	- Parent category
- Instance (name wip)
	- Identifier (add-on to product name)
	- Parent product
- User
	- Name
- Loan
	- Loaner
	- Instance
	- Date
