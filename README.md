A rust base tool for reading data from a csv 
and inserting it into a mysql database for the
White Star Line's revival (Dell Cohort, Team C, 2021)
project.

#### To Create Migrations:
1. Install Rust following instructions from https://rustup.rs/
2. Install Diesel ORM cli 
   
    `cargo install diesel_cli`

3. Add DATABASE_URL to a .env file
4. To generate a migration:
    
    `diesel migration generate <migration name>`
5. Run migrations
    
    `diesel migration run`
6. Test rollback to make sue the `down.sql` file is correct

    `diesel migration redo`