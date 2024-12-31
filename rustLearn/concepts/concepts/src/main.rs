fn main() {
    let tup = ("ICH KANN DAS ALLES NICHT MEHR", 2022);
    let ( message, sub_count) = tup;
    let sub_count = tup.1;
    println!("{}", sub_count);

    let sum = my_function(1123123123123);
    if sum == 0 {
        println!()
    } else if sum == 1 {
        
    }

    let arry = [1,2,3,444,455,234,234,3];
    for element in arry.iter() {
        println!("the value is {}", element);
    }
    /*let mut counter = 0;
    let result: u128 = loop {
        println!("{counter}");
        if counter == 2024 {
            break counter;
        }
        counter += 1;

    };
    println!("Result {}", result);*/

}



fn my_function(x: i128) -> i128 {
    println!("Ich höre {} Stimmen!!!", x);
    x + 2
}
