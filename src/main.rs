mod db;

fn main() {
    match db::setup_database() {
        Ok(_) => println!("Database setup completed successfully."),
        Err(e) => println!("Error setting up de database: {}", e),
    }
}
