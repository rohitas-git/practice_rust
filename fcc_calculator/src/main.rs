use std::env::{args, Args};

// The necessary exports from the env module are the args function, and the Args struct. 
// The args function returns an instance of the Args struct, and is imported into the file scope

// Args struct contains a field called inner which consists of the location of the compiled binary, 
// and the command line arguments passed to the program.


    
fn main() {
    let mut args: Args = args();

    //The args variable needs to be declared as mutable, 
    //because the nth method mutable iterates over the elements, 
    //and removes the element accessed.
    
    // The first argument is the location of the compiled binary, so skip it
    let first: String = args.nth(1).unwrap();
    // After accessing the second argument, the iterator's next element becomes the first
    let operator: String = args.nth(0).unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let operator: char = operator.chars().next().unwrap();

    // println!("{} {} {}", first_number, operator, second_number);
    
    let result = operate(operator, first_number, second_number);
    println!("{}", output(first_number, operator, second_number,result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
      '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
    )
}

