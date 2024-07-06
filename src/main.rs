mod usuario;
mod db;

use db::Database;

fn main() {
    let db = match Database::new("app.db") {
        Ok(db) => db,
        Err(e) => {
            println!("Error connecting to the database: {}", e);
            return;
        }
    };

    if let Err(e) = db.setup() {
        println!("Error setting up the database: {}", e);
        return;
    }

    let conn = db.get_connection();
}
