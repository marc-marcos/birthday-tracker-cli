mod birthday;
mod database;
use database::get_birthdays_from_database;
use rusqlite::{Connection, Result};
use std::fs::File;

fn main() {
    // let conn = database::create_database();
    let conn = database::open_database("database.db");

    let b = birthday::Birthday::build("Otro", "Otros", 28, 9, 2006);

    database::write_to_database(&conn, b);

    get_birthdays_from_database(&conn).unwrap();
}
