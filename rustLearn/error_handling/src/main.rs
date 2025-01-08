use core::{error, panic};
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //let filename = String::from("cargo.lock");
    //let f = File::open(&filename).expect("File couldnt be opened");
    /*let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("") {
                Ok(fc) => fc,
                Err(e) => panic!("FILE KONNTE NIcHT ersStelLt WRRDEN {:?}", e),
            }
            other_error => {
                panic!("PROBLEM !! {:?}", other_error)
            }
        }
    };*/
    let f = File::open("ccargo.lock");
    let mut f = match f {
        Ok(file ) => file,
        Err(e) => panic!("SIGMA {}", e),
    };


    
}