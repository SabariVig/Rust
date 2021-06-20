use std::io;

fn main() {
    let name = input("Enter Your Name").expect("Something Went wrong");

    let age = input("Enter Your Age")
        .expect("Something Went Wrong")
        .parse::<u32>()
        .expect("Failed to convert to number");

    println!("Your Name is {}\nYour Age is {}", name, age)
}

fn input(message: &str) -> io::Result<String> {
    use std::io::Write;
    print!("{}  ", message);
    io::stdout().flush()?;
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
