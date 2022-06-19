use std::{thread, time};

fn main() {

    //let ending = "resources/fathers_day.txt";
    /* let contents = fs::read_to_string(ending)
        .expect("Something went wrong reading the file"); */

    //"..\\resources\\fathers_day.txt";

    let millis = time::Duration::from_millis(4000);

    let millis2 = time::Duration::from_millis(1000);

    let bytes = include_bytes!("..\\resources\\rust_game.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    let bytes = include_bytes!("..\\resources\\plus.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    let bytes = include_bytes!("..\\resources\\rust_logo.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\welcome.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\friend.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\wave.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\hazmat.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\hazmat2.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    println!();
    println!();
    println!();
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\gunshot.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    println!();
    println!();
    println!();
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\standing.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis2);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\crouching.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis2);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\standing.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis2);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\crouching.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis2);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\standing.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis2);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\crouching.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\noob.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);


    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\rust_experience.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

    for i in 1..50 {
        println!();
    }

    let bytes = include_bytes!("..\\resources\\fathers_day.txt");
    print!("{}", String::from_utf8_lossy(bytes));
    thread::sleep(millis);

}
