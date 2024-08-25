use rusqlite::{Connection, Result};

pub fn init_db(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chunks (
            id INTEGER PRIMARY KEY,
            start INTEGER NOT NULL,
            end INTEGER NOT NULL,
            size INTEGER NOT NULL
        )",
        [],
    )?;
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let mut rows = stmt.query(["chunks"])?;
    if (!rows.next()?.is_some()) {
        // insert the chunks
    }
    Ok(conn)
}
