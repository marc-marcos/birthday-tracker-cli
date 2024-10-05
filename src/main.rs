mod birthday;
mod database;
use clap::Parser;
use database::{get_birthdays_from_database, write_to_database};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    a_flag: bool,

    #[arg(short, long)]
    create: bool,

    #[arg(required_if_eq("a_flag", "true"))]
    name: Option<String>,

    #[arg(required_if_eq("a_flag", "true"))]
    surname: Option<String>,

    #[arg(required_if_eq("a_flag", "true"))]
    day: Option<u8>,

    #[arg(required_if_eq("a_flag", "true"))]
    month: Option<u8>,

    #[arg(required_if_eq("a_flag", "true"))]
    year: Option<u16>,
}

fn main() {
    let args = Args::parse();

    let connection = if args.create {
        database::create_database()
    } else {
        database::open_database("database.db")
    };

    if args.a_flag {
        let new_bd: birthday::Birthday = birthday::Birthday::build(
            &args.name.unwrap(),
            &args.surname.unwrap(),
            args.day.unwrap(),
            args.month.unwrap(),
            args.year.unwrap(),
        );

        write_to_database(&connection, new_bd);
    } else {
        get_birthdays_from_database(&connection).unwrap();
    }
}
