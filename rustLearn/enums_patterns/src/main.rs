enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),   
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        let sum = x + y.unwrap_or(0);
        //print!("{}", sum);
    }
    


    {
        value_in_cents(Coin::Quarter(EU_Land::Österreich));

    }

    {
        let one = Some(5);
        let six = plus_eins(one);
        let none = plus_eins(None);
    }

}
fn plus_eins(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(EU_Land),
}
#[derive(Debug)]
enum EU_Land {
    Österreich,
    Deutschland,
    Schweiz,
    Italien,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(land)=> {
            print!("Aus Dem lande {:?}", land);
            25
        },
    }
    
}