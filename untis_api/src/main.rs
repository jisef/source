use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};
use untis::{schools, Error};

fn main() -> Result<(), Error> {
    // Search for a school by name
    let results = schools::search("HTBLA Wels")?;
    let school = match results.first() {
        None => {
            println!("No school found");
            return Ok(());
        },
        Some(v) => v,
    };

    // Log in to the school client
    let mut username = String::new();
    let mut password = String::new();

    {
        let file = File::open("usrname.txt").unwrap();
        let reader = BufReader::new(file);
        if let Some(first_line) = reader.lines().next() {
            username = first_line.unwrap();
        }
    }

    {
        let file = File::open("psswd.txt").unwrap();
        let reader = BufReader::new(file);
        if let Some(first_line) = reader.lines().next() {
            password = first_line.unwrap();
        }
    }




    let mut client = school.client_login(&*username, &*password).unwrap();

    // Fetch the current week's timetable
    let timetable = client.own_timetable_current_week()?;

    // Process or display the timetable data
    println!("{:?}", timetable);

    Ok(())
}
