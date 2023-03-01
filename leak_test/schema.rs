use fstrings::f;
use fstrings::format_args_f;
use marine_sqlite_connector::Connection;

pub const DEFAULT_MAX_ERR_PARTICLES: usize = 50;
pub const DB_FILE: &'static str = "/tmp/db-5.sqlite";

pub fn db() -> Connection {
    // use rand::prelude::*;
    //
    // let db_path = if std::path::Path::new("/tmp/this_is_test").exists() {
    //     format!("/tmp/{}_spell.sqlite", rand::random::<u32>())
    // } else {
    //     format!(DB_FILE)
    // };
    marine_sqlite_connector::open(DB_FILE).expect("open sqlite db")
}

pub fn create() {
    db().execute(
        f!(r#"
            CREATE TABLE IF NOT EXISTS kv (
                key TEXT NOT NULL,
                string TEXT,
                list_index INTEGER DEFAULT -1,

                PRIMARY KEY(key, list_index)
            );
            "#),
    )
        .expect("init sqlite db");
}
