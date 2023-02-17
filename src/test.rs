use marine_rs_sdk::marine;
use marine_sqlite_connector::{version, State, Value};

pub fn main() {}

// Projection with binding in a filter
#[marine]
pub fn test1() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER, weight DOUBLE);
            INSERT INTO users VALUES ('Alice', 42, 100.15);
            INSERT INTO users VALUES ('Bob', 69, 200.11);
        ",
        )
        .unwrap();

    let mut statement = connection
        .prepare("SELECT * FROM users WHERE age > ?")
        .unwrap();

    statement.bind(1, 50).unwrap();

    assert_eq!(statement.next().unwrap(), State::Row);
    assert_eq!(statement.read::<String>(0).unwrap(), "Bob");
    assert_eq!(statement.read::<i64>(1).unwrap(), 69);
    assert_eq!(statement.read::<f64>(2).unwrap(), 200.11);
    assert_eq!(connection.total_changes(), 2);

    let mut statement = connection
        .prepare("SELECT * FROM users WHERE weight > ?")
        .unwrap();

    statement.bind(1, 150.1).unwrap();

    assert_eq!(statement.next().unwrap(), State::Row);
    assert_eq!(statement.read::<String>(0).unwrap(), "Bob");
    assert_eq!(statement.read::<i64>(1).unwrap(), 69);
    assert_eq!(statement.read::<f64>(2).unwrap(), 200.11);
    assert_eq!(connection.total_changes(), 2);

    connection
        .execute("UPDATE users SET age = 20 WHERE age = 69;")
        .unwrap();
    let mut cursor = connection
        .prepare("SELECT * FROM users WHERE age < ?")
        .unwrap()
        .cursor();
    cursor.bind(&[Value::Integer(30)]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[0].as_string().unwrap(), "Bob");
        assert_eq!(row[1].as_integer().unwrap(), 20);
        assert_eq!(row[2].as_float().unwrap(), 200.11);
    }
    assert_eq!(connection.total_changes(), 3);
}

// Another projection with binding in a filter
#[marine]
pub fn test2() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('Bob', 69);
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare("SELECT * FROM users WHERE age > ?")
        .unwrap()
        .cursor();

    cursor.bind(&[Value::Integer(50)]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[0].as_string().unwrap(), "Bob");
        assert_eq!(row[1].as_integer().unwrap(), 69);
    }
    assert_eq!(connection.total_changes(), 2);
}

// BLOB binding
#[marine]
pub fn test3() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE test (number INTEGER, blob BLOB NOT NULL, blob_nullable BLOB);
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare("INSERT OR REPLACE INTO test (number, blob) VALUES (?, ?);")
        .unwrap();

    cursor.bind(1, &Value::Integer(50)).unwrap();
    cursor.bind(2, &Value::Binary(vec![1, 2, 3])).unwrap();

    // check that blob is not null
    assert!(cursor.next().is_ok());
    assert_eq!(connection.total_changes(), 1);
}

// BLOB
#[marine]
pub fn test4() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE test (number INTEGER, blob BLOB);
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare("INSERT OR REPLACE INTO test VALUES (?, ?)")
        .unwrap();

    cursor.bind(1, &Value::Integer(50)).unwrap();
    cursor.bind(2, &Value::Binary(vec![1, 2, 3])).unwrap();

    cursor.next().unwrap();

    let mut cursor = connection
        .prepare("SELECT blob FROM test WHERE number = ?")
        .unwrap()
        .cursor();

    cursor.bind(&[Value::Integer(50)]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[0].as_binary().unwrap().to_vec(), vec![1, 2, 3]);
    }
    assert_eq!(connection.total_changes(), 1);
}

//GROUP BY + ORDER BY
#[marine]
pub fn test5() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('2ndAlice', 42);
            ",
        )
        .unwrap();

    connection
        .execute(
            "
            INSERT INTO users VALUES ('PreAlice', 41);
            INSERT INTO users VALUES ('AliceInAthens', 43);
            INSERT INTO users VALUES ('Bob', 69);
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare("SELECT age, count(*) FROM users GROUP BY age ORDER BY 1")
        .unwrap()
        .cursor();

    let mut res: Vec<i64> = vec![];
    while let Some(row) = cursor.next().unwrap() {
        res.push(row[0].as_integer().unwrap());
    }
    assert_eq!(res, vec![41, 42, 43, 69]);
    // Check the second column
    assert_eq!(connection.total_changes(), 5);
    connection.execute("DROP TABLE users;").unwrap();
    assert_eq!(connection.execute("DROP TABLE users;").is_err(), true);
}

// ORDER BY
#[marine]
pub fn test6() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('2ndAlice', 42);
            INSERT INTO users VALUES ('PreAlice', 41);
            INSERT INTO users VALUES ('AliceInAthens', 43);
            INSERT INTO users VALUES ('Bob', 69);
        ",
        )
        .unwrap();

    let mut statement = connection
        .prepare("SELECT * FROM users WHERE age >= ? ORDER BY age ASC")
        .unwrap();

    statement.bind(1, 43).unwrap();

    assert_eq!(statement.next().unwrap(), State::Row);
    assert_eq!(statement.read::<String>(0).unwrap(), "AliceInAthens");
    assert_eq!(statement.read::<i64>(1).unwrap(), 43);

    assert_eq!(statement.next().unwrap(), State::Row);
    assert_eq!(statement.read::<String>(0).unwrap(), "Bob");
    assert_eq!(statement.read::<i64>(1).unwrap(), 69);
    assert_eq!(connection.total_changes(), 5);
}

// view
#[marine]
pub fn test7() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('2ndAlice', 42);
            INSERT INTO users VALUES ('PreAlice', 41);
            INSERT INTO users VALUES ('AliceInAthens', 43);
            INSERT INTO users VALUES ('Bob', 69);
            CREATE VIEW view1 as SELECT * FROM users WHERE age >= 43;
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare("SELECT name, count(*) FROM view1 GROUP BY name ORDER BY name")
        .unwrap()
        .cursor();
    let golden = vec!["AliceInAthens", "Bob"];
    let mut golden_it = golden.iter();
    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[0].as_string().unwrap(), *golden_it.next().unwrap());
    }
    assert_eq!(connection.total_changes(), 5);
}

#[marine]
pub fn test8() {
    assert!(version() >= 3040001 as usize);
}

// json
#[marine]
pub fn test9() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "CREATE TABLE j2(id INTEGER PRIMARY KEY, json, src);
            INSERT INTO j2(id,json,src)
            VALUES(1,'{
              \"firstName\": \"John\",
              \"lastName\": \"Smith\",
              \"isAlive\": true,
              \"age\": 25,
              \"address\": {
                \"streetAddress\": \"21 2nd Street\",
                \"city\": \"New York\",
                \"state\": \"NY\",
                \"postalCode\": \"10021-3100\"
              },
              \"phoneNumbers\": [
                {
                  \"type\": \"home\",
                  \"number\": \"212 555-1234\"
                },
                {
                  \"type\": \"office\",
                  \"number\": \"646 555-4567\"
                }
              ],
              \"children\": [],
              \"spouse\": null
            }','https://en.wikipedia.org/wiki/JSON');
            INSERT INTO j2(id,json,src)
            VALUES(2, '{
              \"id\": \"0001\",
              \"type\": \"donut\",
              \"name\": \"Cake\",
              \"ppu\": 0.55,
              \"batters\":
                  {
                      \"batter\":
                          [
                              { \"id\": \"1001\", \"type\": \"Regular\" },
                              { \"id\": \"1002\", \"type\": \"Chocolate\" },
                              { \"id\": \"1003\", \"type\": \"Blueberry\" },
                              { \"id\": \"1004\", \"type\": \"Devil''s Food\" }
                          ]
                  },
              \"topping\":
                  [
                      { \"id\": \"5001\", \"type\": \"None\" },
                      { \"id\": \"5002\", \"type\": \"Glazed\" },
                      { \"id\": \"5005\", \"type\": \"Sugar\" },
                      { \"id\": \"5007\", \"type\": \"Powdered Sugar\" },
                      { \"id\": \"5006\", \"type\": \"Chocolate with Sprinkles\" },
                      { \"id\": \"5003\", \"type\": \"Chocolate\" },
                      { \"id\": \"5004\", \"type\": \"Maple\" }
                  ]
             }','https://adobe.github.io/Spry/samples/data_region/JSONDataSetSample.html');
             INSERT INTO j2(id,json,src)
             VALUES(3,'[
              {
                  \"id\": \"0001\",
                  \"type\": \"donut\",
                  \"name\": \"Cake\",
                  \"ppu\": 0.55,
                  \"batters\":
                      {
                          \"batter\":
                              [
                                  { \"id\": \"1001\", \"type\": \"Regular\" },
                                  { \"id\": \"1002\", \"type\": \"Chocolate\" },
                                  { \"id\": \"1003\", \"type\": \"Blueberry\" },
                                  { \"id\": \"1004\", \"type\": \"Devil''s Food\" }
                              ]
                      },
                  \"topping\":
                      [
                          { \"id\": \"5001\", \"type\": \"None\" },
                          { \"id\": \"5002\", \"type\": \"Glazed\" },
                          { \"id\": \"5005\", \"type\": \"Sugar\" },
                          { \"id\": \"5007\", \"type\": \"Powdered Sugar\" },
                          { \"id\": \"5006\", \"type\": \"Chocolate with Sprinkles\" },
                          { \"id\": \"5003\", \"type\": \"Chocolate\" },
                          { \"id\": \"5004\", \"type\": \"Maple\" }
                      ]
              },
              {
                  \"id\": \"0002\",
                  \"type\": \"donut\",
                  \"name\": \"Raised\",
                  \"ppu\": 0.55,
                  \"batters\":
                      {
                          \"batter\":
                              [
                                  { \"id\": \"1001\", \"type\": \"Regular\" }
                              ]
                      },
                  \"topping\":
                      [
                          { \"id\": \"5001\", \"type\": \"None\" },
                          { \"id\": \"5002\", \"type\": \"Glazed\" },
                          { \"id\": \"5005\", \"type\": \"Sugar\" },
                          { \"id\": \"5003\", \"type\": \"Chocolate\" },
                          { \"id\": \"5004\", \"type\": \"Maple\" }
                      ]
              },
              {
                  \"id\": \"0003\",
                  \"type\": \"donut\",
                  \"name\": \"Old Fashioned\",
                  \"ppu\": 0.55,
                  \"batters\":
                      {
                          \"batter\":
                              [
                                  { \"id\": \"1001\", \"type\": \"Regular\" },
                                  { \"id\": \"1002\", \"type\": \"Chocolate\" }
                              ]
                      },
                  \"topping\":
                      [
                          { \"id\": \"5001\", \"type\": \"None\" },
                          { \"id\": \"5002\", \"type\": \"Glazed\" },
                          { \"id\": \"5003\", \"type\": \"Chocolate\" },
                          { \"id\": \"5004\", \"type\": \"Maple\" }
                      ]
              }
             ]','https://adobe.github.io/Spry/samples/data_region/JSONDataSetSample.html');",
        )
        .unwrap();
    let mut cursor = connection
        .prepare("SELECT id, json_valid(json), json_type(json), '|' FROM j2 ORDER BY id;")
        .unwrap()
        .cursor();
    let golden = vec!["object", "object", "array"];
    let mut golden_it = golden.iter();
    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[2].as_string().unwrap(), *golden_it.next().unwrap());
    }

    assert_eq!(connection.total_changes(), 3);
}

// fts5
#[marine]
pub fn test10() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE VIRTUAL TABLE email USING fts5(sender, subject, body);
            INSERT INTO email VALUES ('Alice', 'How deep is the hole', 'Nobody knows');
            INSERT INTO email VALUES ('Wonderland', 'Hatter', 'Normality. I know this');
            INSERT INTO email VALUES ('Rabbit the optimist', 'He never sleeps', 'Never I said you know');
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare("SELECT sender FROM email WHERE body MATCH 'know';")
        .unwrap()
        .cursor();
    let golden = vec!["Wonderland", "Rabbit the optimist"];
    let mut golden_it = golden.iter();
    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[0].as_string().unwrap(), *golden_it.next().unwrap());
    }
}

// rtree
#[marine]
pub fn test11() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE VIRTUAL TABLE demo_index USING rtree(
                id,              -- Integer primary key
                minX, maxX,      -- Minimum and maximum X coordinate
                minY, maxY       -- Minimum and maximum Y coordinate
             );
             INSERT INTO demo_index VALUES
                (28215, -80.781227, -80.604706, 35.208813, 35.297367),
                (28216, -80.957283, -80.840599, 35.235920, 35.367825),
                (28217, -80.960869, -80.869431, 35.133682, 35.208233),
                (28226, -80.878983, -80.778275, 35.060287, 35.154446),
                (28227, -80.745544, -80.555382, 35.130215, 35.236916),
                (28244, -80.844208, -80.841988, 35.223728, 35.225471),
                (28262, -80.809074, -80.682938, 35.276207, 35.377747),
                (28269, -80.851471, -80.735718, 35.272560, 35.407925),
                (28270, -80.794983, -80.728966, 35.059872, 35.161823),
                (28273, -80.994766, -80.875259, 35.074734, 35.172836),
                (28277, -80.876793, -80.767586, 35.001709, 35.101063),
                (28278, -81.058029, -80.956375, 35.044701, 35.223812),
                (28280, -80.844208, -80.841972, 35.225468, 35.227203),
                (28282, -80.846382, -80.844193, 35.223972, 35.225655);
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare(
            "SELECT A.id FROM demo_index AS A, demo_index AS B
                    WHERE A.maxX>=B.minX AND A.minX<=B.maxX
                    AND A.maxY>=B.minY AND A.minY<=B.maxY
                    AND B.id=28269 ORDER BY 1;",
        )
        .unwrap()
        .cursor();
    let golden = vec![28215, 28216, 28262, 28269];
    let mut golden_it = golden.iter();
    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[0].as_integer().unwrap(), *golden_it.next().unwrap());
    }
}

// triggers
#[marine]
pub fn test12() {
    let connection = marine_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE leads (
                id integer PRIMARY KEY,
                first_name text NOT NULL,
                last_name text NOT NULL,
                phone text NOT NULL,
                email text NOT NULL
            );
        ",
        )
        .unwrap();

    connection
        .execute(
            "
            CREATE TRIGGER validate_email_before_insert_leads
            BEFORE INSERT ON leads
            BEGIN
            SELECT
                CASE
                WHEN NEW.email NOT LIKE '%_@__%.__%' THEN
                RAISE (ABORT,'Invalid email address')
                END;
            END;
        ",
        )
        .unwrap();

    connection
        .execute(
            "
            INSERT INTO leads (first_name, last_name, email, phone)
            VALUES ('John', 'Doe', 'john.doe@sqlitetutorial.net', '4089009334');
        ",
        )
        .unwrap();

    assert!(connection
        .execute(
            "
            INSERT INTO leads (first_name,last_name,email,phone)
            VALUES('John','Doe','jjj','4089009334');
        ",
        )
        .is_err());

    let mut cursor = connection
        .prepare(
            "
            SELECT first_name, last_name, email FROM leads;
            ",
        )
        .unwrap()
        .cursor();
    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[0].as_string().unwrap(), "John");
        assert_eq!(row[2].as_string().unwrap(), "john.doe@sqlitetutorial.net");
    }
    // ON UPDATE
    connection
        .execute(
            "
            CREATE TABLE lead_logs (
                id INTEGER PRIMARY KEY,
                old_id int,
                new_id int,
                old_phone text,
                new_phone text,
                old_email text,
                new_email text,
                user_action text,
                created_at text
            );
        ",
        )
        .unwrap();

    connection
        .execute(
            "
            CREATE TRIGGER log_contact_after_update
            AFTER UPDATE ON leads
            WHEN old.phone <> new.phone
                 OR old.email <> new.email
         BEGIN
             INSERT INTO lead_logs (
                 old_id,
                 new_id,
                 old_phone,
                 new_phone,
                 old_email,
                 new_email,
                 user_action,
                 created_at
             )
         VALUES
             (
                 old.id,
                 new.id,
                 old.phone,
                 new.phone,
                 old.email,
                 new.email,
                 'UPDATE',
                 DATETIME('NOW')
             ) ;
         END;
        ",
        )
        .unwrap();

    connection
        .execute(
            "
            UPDATE leads
            SET
               last_name = 'Smith'
            WHERE
               id = 1;
        ",
        )
        .unwrap();
    let mut statement = connection
        .prepare(
            "
            SELECT count(*) FROM lead_logs;
            ",
        )
        .unwrap();

    assert_eq!(statement.next().unwrap(), State::Row);
    assert_eq!(statement.read::<i64>(0).unwrap(), 0);
    connection
        .execute(
            "
        UPDATE leads
        SET
           phone = '4089998888',
           email = 'john.smith@sqlitetutorial.net'
        WHERE
           id = 1;
        ",
        )
        .unwrap();
    let mut cursor = connection
        .prepare(
            "
        SELECT old_phone,new_phone,old_email,new_email,user_action
        FROM lead_logs;
        ",
        )
        .unwrap()
        .cursor();
    while let Some(row) = cursor.next().unwrap() {
        assert_eq!(row[0].as_string().unwrap(), "4089009334");
        assert_eq!(row[1].as_string().unwrap(), "4089998888");
        assert_eq!(row[2].as_string().unwrap(), "john.doe@sqlitetutorial.net");
        assert_eq!(row[3].as_string().unwrap(), "john.smith@sqlitetutorial.net");
        assert_eq!(row[4].as_string().unwrap(), "UPDATE");
    }
}
