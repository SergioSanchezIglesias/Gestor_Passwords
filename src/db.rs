use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Database { conn })
    }

    pub fn setup(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Usuarios (
              UserID INTEGER PRIMARY KEY AUTOINCREMENT,
              Username TEXT NOT NULL UNIQUE,
              Password TEXT NOT NULL
          )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Contraseñas (
              PasswordID INTEGER PRIMARY KEY AUTOINCREMENT,
              UserID INTEGER NOT NULL,
              Nombre_del_Servicio TEXT NOT NULL,
              Username TEXT NOT NULL,
              Password TEXT NOT NULL,
              FOREIGN KEY(UserID) REFERENCES Usuarios(UserID)
          )",
            [],
        )?;
        Ok(())
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    pub fn initialize(db_path: &str) -> Result<Self, String> {
        let db = Database::new(db_path)
            .map_err(|e| format!("Error connecting to the database: {}", e))?;
        db.setup()
            .map_err(|e| format!("Error setting up the database: {}", e))?;
        Ok(db)
    }
}
