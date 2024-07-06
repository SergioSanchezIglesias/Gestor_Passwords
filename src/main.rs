mod db;
mod funciones;
mod usuario;

use db::Database;
use rusqlite::Connection;
use std::env;
use usuario::UserManager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !funciones::args_validation(&args) {
        return;
    }

    if funciones::args_management(&args) {
        return;
    }

    let db = match Database::initialize("app.db") {
        Ok(db) => db,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let conn: &Connection = db.get_connection();
    let user_manager: UserManager = UserManager::new(conn);

    loop {
        funciones::show_menu();

        let choice = funciones::get_option();

        match choice {
            1 => {
                println!("Función añadir usuarios no implementada");
            }

            2 => {
                println!("Función autenticar usuario no implementada")
            }

            3 => break,
            _ => println!("Opción no válida. Introduce una opción correcta."),
        }
    }
}
