use std::io::stdin;
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

    let username = stdin().read_line().expect();
    // Log in to the school client
    let mut client = school.client_login("username", "password")?;

    // Fetch the current week's timetable
    let timetable = client.own_timetable_current_week()?;

    // Process or display the timetable data
    println!("{:?}", timetable);

    Ok(())
}
