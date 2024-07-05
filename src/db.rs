use rusqlite::{Connection, Result};

pub fn connect_db() -> Result<Connection> {
    Connection::open("app.db").map_err(|e| {
        eprintln!("Error al conectar con la base de datos: {}", e);
        e
    })
}

pub fn setup_database() -> Result<()> {
    let conn = connect_db()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Usuarios (
          UserID INTEGER PRIMARY KEY AUTOINCREMENT,
          Username TEXT NOT NULL UNIQUE,
          Password TEXT NOT NULL
      )",
        [],
    )
    .map_err(|e| {
        eprintln!("Error al crear la tabla Usuarios: {}", e);
        e
    })?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Contraseñas (
          PasswordID INTEGER PRIMARY KEY AUTOINCREMENT,
          UserID INTEGER NOT NULL,
          Nombre_del_Servicio TEXT NOT NULL,
          Username TEXT NOT NULL,
          Password TEXT NOT NULL,
          FOREIGN KEY(UserID) REFERENCES Usuarios(UserID)
      )",
        [],
    )
    .map_err(|e| {
        eprintln!("Error al crear la tabla Contraseñas: {}", e);
        e
    })?;

    Ok(())
}
