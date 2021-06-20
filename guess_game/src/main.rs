use rand::Rng;
use std::io;

fn main() {
    let rand_number = rand::thread_rng().gen_range(0..10);
    // print!("The Random numbre is {} ", num);
    let mut counter = 0;
    loop {
        let guess_number = input("Enter A Number:")
            .expect("Unexpecetd Error")
            .parse::<u32>()
            .expect("Unable to parse as u32");
        counter += 1;
        if guess_number == rand_number {
            println!("You successfully Gussed the number in {} gusses", counter);
            break;
        }
    }
}

fn input(message: &str) -> io::Result<String> {
    use std::io::Write;
    print!("{} ", message);
    io::stdout().flush()?;
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
