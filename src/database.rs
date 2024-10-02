use rusqlite::{Connection, Result};

use crate::birthday;

pub fn create_database() -> Connection {
    let connection = Connection::open("database.db").unwrap();

    connection
        .execute(
            "
            CREATE TABLE Birthday (
            name TEXT,
            surname TEXT,
            day INTEGER,
            month INTEGER,
            year INTEGER
        );
        ",
            (),
        )
        .unwrap();

    connection
}

pub fn open_database(path: &str) -> Connection {
    Connection::open(path).unwrap()
}

pub fn write_to_database(conn: &Connection, obj: birthday::Birthday) {
    conn.execute(
        "
                INSERT INTO Birthday (name, surname, day, month, year) VALUES
                (?1, ?2, ?3, ?4, ?5)
             ",
        (obj.name, obj.surname, obj.day, obj.month, obj.year),
    )
    .unwrap();
}

pub fn get_birthdays_from_database(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name, surname, day, month, year FROM Birthday")?;

    let birthdays = stmt.query_map([], |row| {
        Ok(birthday::Birthday {
            name: row.get(0)?,
            surname: row.get(1)?,
            day: row.get(2)?,
            month: row.get(3)?,
            year: row.get(4)?,
        })
    })?;

    for birthday in birthdays {
        match birthday {
            Ok(bday) => println!("Found a birthday: {}", bday.name),
            Err(e) => eprintln!("Error reading birthday: {}", e),
        }
    }

    Ok(())
}
