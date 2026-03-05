use super::models::DatabaseConfig;
use rusqlite::{Connection, Result as SqliteResult};

pub fn init_database(config: &DatabaseConfig) -> SqliteResult<Connection> {
    let conn = Connection::open(&config.path)?;

    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;
        PRAGMA busy_timeout = 5000;
        PRAGMA synchronous = NORMAL;
        ",
    )?;

    create_schema(&conn)?;
    create_indices(&conn)?;

    Ok(conn)
}

pub fn create_schema(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS participants (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            role TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            participant_id INTEGER NOT NULL,
            category TEXT NOT NULL,
            description TEXT NOT NULL,
            date_emission TEXT NOT NULL,
            date_due TEXT NOT NULL,
            date_payment TEXT,
            value_agreed INTEGER NOT NULL,
            value_paid INTEGER NOT NULL,
            installment_current INTEGER NOT NULL,
            installment_total INTEGER NOT NULL,
            payment_method TEXT NOT NULL,
            status TEXT NOT NULL,
            observations TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (participant_id) REFERENCES participants(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            transaction_id INTEGER,
            action TEXT NOT NULL,
            participant_id INTEGER NOT NULL,
            changes TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE SET NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn create_indices(conn: &Connection) -> SqliteResult<()> {
    conn.execute("CREATE INDEX IF NOT EXISTS idx_transactions_participant_id ON transactions(participant_id)", [])?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transactions_date_due ON transactions(date_due)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transactions_status ON transactions(status)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transactions_date_emission ON transactions(date_emission)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_audit_log_transaction_id ON audit_log(transaction_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp ON audit_log(timestamp)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_audit_log_participant_id ON audit_log(participant_id)",
        [],
    )?;

    Ok(())
}

pub fn database_exists(config: &DatabaseConfig) -> bool {
    std::path::Path::new(&config.path).exists()
}

pub fn ensure_database_exists(config: &DatabaseConfig) -> SqliteResult<Connection> {
    if !database_exists(config) {
        let conn = init_database(config)?;
        Ok(conn)
    } else {
        let conn = Connection::open(&config.path)?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        conn.execute("PRAGMA busy_timeout = 5000;", [])?;
        Ok(conn)
    }
}

pub fn create_schema_in_memory() -> SqliteResult<Connection> {
    let conn = Connection::open(":memory:")?;
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;
        PRAGMA busy_timeout = 5000;
        PRAGMA synchronous = NORMAL;
        ",
    )?;
    create_schema(&conn)?;
    create_indices(&conn)?;
    Ok(conn)
}

pub fn setup_memory_db() -> SqliteResult<Connection> {
    let conn = Connection::open(":memory:")?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    conn.execute("PRAGMA busy_timeout = 5000;", [])?;
    Ok(conn)
}
