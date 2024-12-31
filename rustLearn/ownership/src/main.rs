fn main() {
    let x = 5;
    let y = x; // x is copied

    println!("Hello, world!");
    let s1: String = String::from("Hello");
    let s2: String = s1.clone(); // if = s1 -> onwership transferred
    print!("{}", s1);




    let s = String::from("Sigma");
    takes_ownership(s.clone());
    println!("{}",s);

    let i: i32 = 4;
    copy(i);
    println!("{i}");


    let s = String::from("Toilet");
    let len = gives_onwershop(&s);
    println!("s: {}", s);
    println!("the lenght of is {len}");
    println!("**********************");


    let mut newString = String::from("DONGLE");
    chnage(&mut newString);
    println!("THE STRING: {}", newString);
    println!("**********************");


    let f = String::from("123");
    let xx = &f;
    let yy = &f;

    println!("{},{}", xx, yy);

}

fn takes_ownership(some: String) {
    println!("Takes Ownership: {}", some);
}

fn copy(some:i32) {
    println!("COPY {}" , some);
}


fn gives_onwershop (s:&String) -> usize {
    let length = s.len();
        length
}

fn chnage(stringofTrust:&mut String) {
    stringofTrust.push_str("awdad");
}

