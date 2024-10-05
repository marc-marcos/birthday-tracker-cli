use crate::birthday;
use chrono::Datelike;
use rusqlite::{params, Connection, Result};

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
    let current_date = chrono::Utc::now();
    let one_week = chrono::TimeDelta::days(7);
    let final_date = current_date + one_week;

    // let mut stmt = conn.prepare("SELECT name, surname, day, month, year FROM Birthday WHERE () AND ()")?;
    let mut stmt = conn.prepare(
        "SELECT name, surname, day, month, year 
         FROM Birthday
         WHERE 
            (
                -- If birthdays fall between today and the limit within the same year
                year = ?1 AND
                (
                    (month > ?2 OR (month = ?2 AND day >= ?3))
                    AND 
                    (month < ?4 OR (month = ?4 AND day <= ?5))
                )
            )
            OR
            (
                -- If today is in the current year and the limit is in the next year
                year = ?1 AND
                (month > ?2 OR (month = ?2 AND day >= ?3))
            )
            OR
            (
                -- If the birthday is in the next year, before the limit date
                year = ?6 AND
                (month < ?4 OR (month = ?4 AND day <= ?5))
            )",
    )?;

    let birthday_iter = stmt.query_map(
        params![
            current_date.year(),
            current_date.month(),
            current_date.day(),
            current_date.month(),
            current_date.day(),
            current_date.year()
        ],
        |row| {
            Ok((
                row.get(0)?, // name
                row.get(1)?, // surname
                row.get(2)?, // day
                row.get(3)?, // month
                row.get(4)?, // year
            ))
        },
    )?;

    let final_vec: Vec<birthday::Birthday> = vec![];

    for result in birthday_iter {
        // Handle the Result properly
        match result {
            Ok((name, surname, day, month, year)) => {
                // Push a new Birthday struct into the vector
                final_vec.push(birthday::Birthday::build(name?, surname?, day, month, year));
            }
            Err(e) => {
                // Handle the error case appropriately (e.g., log the error or return it)
                eprintln!("Error fetching birthday: {}", e);
                return Err(e); // Optionally return the error here
            }
        }
    }

    for birthday in final_vec {
        dbg!(birthday);
    }

    Ok(())
}
