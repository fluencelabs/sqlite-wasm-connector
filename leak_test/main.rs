#![feature(try_blocks)]

use marine_rs_sdk::marine;
use marine_sqlite_connector::State;

mod schema;

use fstrings::f;
use schema::db;
use schema::get_storage;
use schema::wrapped_try;
use serde::Deserialize;
use serde::Serialize;

fn main() {}

#[marine]
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct UnitValue {
    pub success: bool,
    pub error: String,
}

#[marine]
pub fn create_4(path: String) {
    schema::create();
    /*
    let conn_create = marine_sqlite_connector::open(&path).expect("Open database connection");

    conn_create
        .execute(
            "
            CREATE TABLE IF NOT EXISTS trigger_config (
                -- clock config
                start_sec INTEGER, end_sec INTEGER, period_sec INTEGER,
                -- connection pool config
                connect INTEGER, disconnect INTEGER,
                -- blockchain config
                start_block INTEGER, end_block INTEGER
            );
            CREATE TABLE IF NOT EXISTS relay (relay TEXT);
            -- CREATE TABLE IF NOT EXISTS kv (key TEXT, string TEXT, u32 INTEGER, list_index INTEGER);
            CREATE TABLE IF NOT EXISTS kv (
                key TEXT NOT NULL,
                string TEXT,
                u32 INTEGER,
                list_index INTEGER DEFAULT -1,
                PRIMARY KEY(key, list_index)
            );
            -- particles stored in the database, LRU-like
            CREATE TABLE IF NOT EXISTS particles (particle_id TEXT PRIMARY KEY, timestamp INTEGER);
            -- errors happened in particles
            CREATE TABLE IF NOT EXISTS errors (
                particle_id TEXT,
                timestamp INTEGER,
                error_idx INTEGER,
                error_code INTEGER,
                instruction TEXT,
                message TEXT,
                peer_id TEXT
            );
            CREATE TABLE IF NOT EXISTS particle_count (parameter TEXT PRIMARY KEY, value INTEGER NOT NULL);
            -- maximum number of particles to store information about
            INSERT OR REPLACE INTO particle_count VALUES ('max_particles', 50);
            -- current count of stored particles
            INSERT OR REPLACE INTO particle_count VALUES ('count_particles', 0);
            -- if there are more than `max_particles` particles, delete the oldest one
            CREATE TRIGGER IF NOT EXISTS errors_limit_trigger AFTER INSERT ON particles
                FOR EACH ROW
                -- if limit is reached
                WHEN (SELECT value FROM particle_count WHERE parameter = 'count_particles')
                    > (SELECT value FROM particle_count WHERE parameter = 'max_particles')
                BEGIN
                    -- delete all errors for the oldest particle
                    DELETE FROM particles
                        -- take oldest by 'timestamp' column
                        WHERE particle_id = (SELECT particle_id FROM particles ORDER BY timestamp LIMIT 1);
                END;
            -- when a particle is removed, remove its errors
            CREATE TRIGGER IF NOT EXISTS clear_errors AFTER DELETE ON particles
                FOR EACH ROW
                BEGIN
                    -- remove all errors for that particle
                    DELETE FROM errors WHERE particle_id = OLD.particle_id;
                    -- decrement number of particles
                    UPDATE particle_count SET value = value - 1 WHERE parameter = 'count_particles';
                END;
            -- when a particle is inserted, incremenet the counter
            CREATE TRIGGER IF NOT EXISTS particles_count_insert_trigger AFTER INSERT ON particles
                FOR EACH ROW
                BEGIN
                  UPDATE particle_count SET value = value + 1 WHERE parameter = 'count_particles';
                END;
            -- when a particle error is inserted, store particle id if it wasn't there yet
            CREATE TRIGGER IF NOT EXISTS store_particle_id AFTER INSERT ON errors
                FOR EACH ROW
                BEGIN
                    INSERT OR IGNORE INTO particles (particle_id, timestamp) VALUES (NEW.particle_id, NEW.timestamp);
                END;
        ",
        )
        .expect("running schema queries");
     */
}

#[marine]
fn insert_1(path: String) {
    let conn = marine_sqlite_connector::open(path).expect("Open database connection");

    let key = "some";
    let value = "other";
    let mut statement = conn
        .prepare("INSERT OR REPLACE INTO kv (key, string) VALUES (?, ?)")
        .expect("prep rand 0..3");
    statement.bind(1, key).expect("bind 1");
    statement.bind(2, value).expect("bind 2");
    statement.next().expect("next");
}

#[marine]
fn insert_2(path: String) {
    let conn = marine_sqlite_connector::open(path).expect("Open database connection");

    let mut statement = conn
        .prepare("INSERT OR REPLACE INTO kv (key, u32) VALUES (?, ?)")
        .expect("prep rand 6..7");
    statement.bind(1, 42).expect("6..7 bind");
    statement.bind(2, 42).expect("6..7 bind");
    statement.next().expect("4..5 bind");
    if let State::Row = statement.next().expect("6..7 next") {
        statement.read::<String>(0).expect("6..7 read");
    }
}

#[marine]
fn insert_3() {
    let conn = db();

    let key = "some";
    let value = "other";
    let mut statement = conn
        .prepare("INSERT OR REPLACE INTO kv (key, string) VALUES (?, ?)")
        .expect("prep rand 0..3");
    statement.bind(1, key).expect("bind 1");
    statement.bind(2, value).expect("bind 2");
    statement.next().expect("next");
}

#[marine]
fn insert_4(key: &str, value: String, i: u32) {
    let conn = db();

    let mut statement = conn
        .prepare("INSERT OR REPLACE INTO kv (key, string) VALUES (?, ?)")
        .expect("prep rand 0..3");
    statement.bind(1, key).expect("bind 1");
    statement.bind(2, value.as_str()).expect("bind 2");
    statement.next().expect("next");
    // if i % 10000 == 0 {
    //     println!(
    //         "sqlite3 memory_used - {:}",
    //         marine_sqlite_connector::status(0, 0)
    //     );
    //     println!(
    //         "sqlite3 pagecache_used - {:}",
    //         marine_sqlite_connector::status(1, 0)
    //     );
    // }
}

#[marine]
fn insert_5(key: &str, value: String) {
    let mut statement = db()
        .prepare("INSERT OR REPLACE INTO kv (key, string) VALUES (?, ?)")
        .expect("prep rand 0..3");
    statement.bind(1, key).expect("bind 1");
    statement.bind(2, value.as_str()).expect("bind 2");
    statement.next().expect("next");
}

#[marine]
fn insert_6(key: &str, value: String, i: u32) {
    wrapped_try(|| {
        let storage = get_storage().unwrap();
        let mut statement = storage
            .connection
            .prepare("INSERT OR REPLACE INTO kv (key, string) VALUES (?, ?)")
            .expect("prep rand 0..3");
        statement.bind(1, key).expect("bind 1");
        statement.bind(2, value.as_str()).expect("bind 2");
        statement.next().expect("next");
    });
    // if i % 10000 == 0 {
    //     println!(
    //         "sqlite3 memory_used - {:}",
    //         marine_sqlite_connector::status(0, 0)
    //     );
    //     println!(
    //         "sqlite3 pagecache_used - {:}",
    //         marine_sqlite_connector::status(1, 0)
    //     );
    // }
}

/*
#[marine]
pub fn list_push_string_5(key: &str, value: String) {
    let mut statement = db().prepare(
        r#"
                INSERT INTO kv (key, string)
                    VALUES (?, ?)
            "#,
    ).unwrap();
    statement.bind(1, key).unwrap();
    statement.bind(2, value.as_str()).unwrap();
    statement.next().unwrap();
}
 */

#[marine]
fn select_1(i: u32) {
    let conn = marine_sqlite_connector::open("./tmp/db.sqlite").expect("Open database connection");

    let mut statement = conn
        .prepare("SELECT string FROM kv WHERE key = ?")
        .expect("prep rand 4..5");
    let key = "some";
    statement.bind(1, key).expect("4..5 bind");
    if let State::Row = statement.next().expect("4..5 next") {
        statement.read::<String>(0).expect("4..5 read");
    }
    // if i % 10000 == 0 {
    //     println!(
    //         "sqlite3 memory_used - {:}",
    //         marine_sqlite_connector::status(0, 0)
    //     );
    //     println!(
    //         "sqlite3 pagecache_used - {:}",
    //         marine_sqlite_connector::status(1, 0)
    //     );
    // }
}

#[marine]
fn select_5() {
    let conn = marine_sqlite_connector::open("./tmp/db.sqlite").expect("Open database connection");
}

#[marine]
fn set_limit(limit: i64) -> i64 {
    marine_sqlite_connector::set_hard_memory_limit(limit)
}

#[marine]
pub fn list_push_string_1(key: &str, value: String) -> UnitValue {
    let result: eyre::Result<()> = try {
        let mut statement = db().prepare(
            r#"
                INSERT INTO kv (key, string, list_index)
                    VALUES (
                        ?,
                        ?,
                        42
                    )
            "#,
        )?;
        statement.bind(1, key)?;
        statement.bind(2, value.as_str())?;
        //statement.bind(3, key)?;
        statement.next()?;
    };

    result.into()
}

#[marine]
pub fn list_push_string_2(key: &str, value: String) -> UnitValue {
    let mut statement = db()
        .prepare(
            r#"
                INSERT INTO kv (key, string, list_index)
                    VALUES (
                        ?,
                        ?,
                        42
                    )
            "#,
        )
        .unwrap();
    statement.bind(1, key).unwrap();
    statement.bind(2, value.as_str()).unwrap();
    statement.next().unwrap();

    UnitValue::ok()
}

#[marine]
pub fn list_push_string_3(key: &str, value: String) {
    let mut statement = db()
        .prepare(
            r#"
                INSERT INTO kv (key, string, list_index)
                    VALUES (
                        ?,
                        ?,
                        42
                    )
            "#,
        )
        .unwrap();
    statement.bind(1, key).unwrap();
    statement.bind(2, value.as_str()).unwrap();
    statement.next().unwrap();
}

#[marine]
pub fn list_push_string_4(key: &str, value: String) {
    let mut statement = db()
        .prepare(
            r#"
                INSERT INTO kv (key, string)
                    VALUES (
                        ?,
                        ?
                    )
            "#,
        )
        .unwrap();
    statement.bind(1, key).unwrap();
    statement.bind(2, value.as_str()).unwrap();
    statement.next().unwrap();
}

#[marine]
pub fn list_push_string_5(key: &str, value: String) {
    let mut statement = db()
        .prepare(
            r#"
                INSERT INTO kv (key, string)
                    VALUES (?, ?)
            "#,
        )
        .unwrap();
    statement.bind(1, key).unwrap();
    statement.bind(2, value.as_str()).unwrap();
    statement.next().unwrap();
}

#[marine]
pub fn list_push_string_6(key: &str, value: String) {
    let mut statement = db()
        .prepare(
            r#"
                INSERT OR REPLACE INTO kv (key, string)
                    VALUES (?,?)
            "#,
        )
        .unwrap();
    statement.bind(1, key).unwrap();
    statement.bind(2, value.as_str()).unwrap();
    statement.next().unwrap();
}

impl UnitValue {
    pub fn ok() -> Self {
        Self {
            success: true,
            error: String::new(),
        }
    }

    pub fn error(error: impl AsRef<str>) -> Self {
        Self {
            success: false,
            error: error.as_ref().to_string(),
        }
    }

    pub fn spell_error(error: SpellError) -> Self {
        Self::error(format_error(error))
    }
}

impl From<eyre::Result<()>> for UnitValue {
    fn from(value: eyre::Result<()>) -> Self {
        match value {
            Ok(_) => UnitValue::ok(),
            Err(e) => UnitValue::error(format_error(e)),
        }
    }
}

use marine_sqlite_connector::Error as SqliteError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum SpellError {
    #[error("Internal Sqlite error: {0}")]
    SqliteError(
        #[from]
        #[source]
        SqliteError,
    ),
    #[error("Key '{0}' does not exist")]
    KeyNotExists(String),
    #[error("Location not available: relay was not set")]
    NoRelay,
    #[error("Only owner of the spell can set relay peer id")]
    SetRelayForbidden,
    #[error("Relay was already set and cannot be changed")]
    RelayAlreadySet,
    #[error("Only owner of the spell can set trigger config")]
    SetTriggerConfigForbidden,
    #[error("Trigger Config is not set. Use set_trigger_config to set it.")]
    NoTriggerConfig,
}

pub fn format_error(e: impl std::fmt::Debug) -> String {
    format!("{:?}", e)
}
