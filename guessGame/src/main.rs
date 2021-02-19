use rand::Rng;
use std::io;

fn main() {
    let randomnumber = rand::thread_rng().gen_range(0..10);
    println!("{}",randomnumber);
    let mut counter = 1;
    loop {
        println!("Enter A Number");
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Unable To Read From Stdin");
        let input_text = input_text.trim().parse::<i32>().expect("Invalid Input");
        if input_text == randomnumber {
            break;
        } else {
            counter += 1;
        }
    }
    println!("You Gussed In {} Tries", counter)
}
