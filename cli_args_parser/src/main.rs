// Pull std::env into scope
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Print number of arguments
    println!("Found {} arguments.", args.len());

    // Print the args list. Pretty print by using "{:#?}""
    println!("{:?}", args);

    // Collect arg in a var
    let arg0 = &args[0];
    println!("Arg 0: {}", arg0);

    // Print args by iteration
    for arg in &args {
        println!("{:?}", arg);
    }
}
