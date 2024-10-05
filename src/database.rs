use crate::birthday;
use chrono::Datelike;
use rusqlite::{params, Connection, Result};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
            -- Check if birthday falls between today and the limit within the same year
            (month > ?1 OR (month = ?1 AND day >= ?2))
            AND 
            (month < ?3 OR (month = ?3 AND day <= ?4))
        )
        OR
        (
            -- If today is in the current year and the limit is in the next year
            (month > ?1 OR (month = ?1 AND day >= ?2))
        )
        OR
        (
            -- If the birthday is in the next year, before the limit date
            (month < ?3 OR (month = ?3 AND day <= ?4))
        )     
        ORDER BY 
        (month - ?1) * 31 + (day - ?2) ASC",
    )?;

    let birthday_iter = stmt.query_map(
        params![
            current_date.month(),
            current_date.day(),
            final_date.month(),
            final_date.day(),
        ],
        |row| {
            Ok(birthday::Birthday {
                name: row.get(0)?,    // name
                surname: row.get(1)?, // surname
                day: row.get(2)?,     // day
                month: row.get(3)?,   // month
                year: row.get(4)?,    // year
            })
        },
    )?;

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for birthday in birthday_iter {
        let current_bd = birthday.unwrap();

        // Bold and pretty printing
        if current_bd.day as u32 == current_date.day()
            && current_bd.month as u32 == current_date.month()
        {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
                .unwrap();
            writeln!(
                &mut stdout,
                "{}/{} - {} {}",
                current_bd.day, current_bd.month, current_bd.name, current_bd.surname
            )
            .unwrap();
            stdout.reset().unwrap();
        } else {
            writeln!(
                &mut stdout,
                "{}/{} - {} {}",
                current_bd.day, current_bd.month, current_bd.name, current_bd.surname
            )
            .unwrap();
        }
    }

    Ok(())
}
