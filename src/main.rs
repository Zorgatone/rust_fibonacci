fn main() {
    println!("Which index in the fibonacci do you want to calculate?");

    let mut response = String::new();

    std::io::stdin().read_line(&mut response)
        .expect("Cannot read line!");

    let index: u32 = match response.trim().parse() {
        Ok(n) => if n > 0 {
            n
        } else {
            eprintln!("Number must be greater than 0!");
            return
        },
        Err(_) => {
            eprintln!("Please enter a number!");
            return
        }
    };

    println!("The #{} fibonacci number is {}", index, fib(index - 1));
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}
