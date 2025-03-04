use std::io::{self, Write};
use std::panic::panic_any;

use clap::{Arg, ArgMatches, Command, Parser};
use untis::{schools::search, School};

fn main() {
    let matches = Command::new("untis")
        .about("SIGMA")
        .author("jisef")
        .arg(
            Arg::new("end-of-lesson")
                .help("Shows the end of a lesson")
                .long("end-of-lesson")
                .short('e')
                .num_args(0),
        )
        .arg(
            Arg::new("password")
                .help("Password for untis api")
                .short('p')
                .long("password")
                .num_args(1),
        )
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .help("Username for untis api"),
        )
        .get_matches();
    let username = if matches.get_one::<String>("username").is_none() {
        println!("Username");
        print!("> ");
        let _ = io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        input = input.to_string();
        input
    } else {
        matches.get_one::<String>("username").unwrap().to_string()
    };

    let password = if matches.get_one::<String>("password").is_none() {
        println!("Password");
        print!("> ");
        let _ = io::stdout().flush();
        match rpassword::read_password() {
            Ok(password) => {
                println!("Password entered successfully!");
                password
            }
            Err(err) => {
                panic!("Password input failed: {}", err.to_string())
            }
        }
    } else {
        matches.get_one::<String>("password").unwrap().to_string()
    };

    let results = untis::schools::search("HTBLA Wels").unwrap();
    let school = match results.first() {
        Some(x) => x,
        None => panic!("SCHULE NICHT GEFUNDEN ODER SO"),
    };

    println!("SCHOOL: {}", school.address);
    println!("Server: {}", school.server);
    //println!("pw: '{}'", password);
    println!("USERNAME : {}", &*username);

    let mut client = match school.client_login(&*username, &*password) {
        Ok(x) => x,
        Err(e) => panic!("Login failed!: {}", e.to_string()),
    };

    let date = chrono::Local::now().date_naive() + chrono::Duration::weeks(2);

    // Get the client's own timetable until 2 weeks from now.
    let timetable = client.own_timetable_until(&date.into()).unwrap();

    for lesson in timetable {
        println!("{:?}", lesson);
    }
}
