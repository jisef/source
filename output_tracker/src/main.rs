use crate::output_type::Essen;

mod db_manager;

#[derive(Debug, Clone)]
enum output_type {
    Hygiene,
    Essen,
    Freizeit
}


fn main() {
    println!("Hello, world!");
    let symlink_dir = output_type::Essen;
    print!("{:?}", symlink_dir);
}
