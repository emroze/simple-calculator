use std::env;

fn calculate(val1:f64,val2:f64,operator:char) -> f64{
    match operator{
        '+' => val1 + val2,
        '-' => val1 - val2,
        '/' => val1 / val2,
        '*' | 'x' | 'X' => val1 * val2,
        _ => panic!("Invalid operator used."),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()!=4 {
        panic!("Not enough or too much arguments. Aborting!!!")
    }

    let first_value = &args[1];
    let first_value = first_value.parse::<f64>().expect("Invalid argument. Aborting!!!");
    let operator = String::from(&args[2]).chars().next().unwrap();
    let second_value = &args[3];
    let second_value = second_value.parse::<f64>().expect("Invalid argument. Aborting!!!");

    let result = calculate(first_value,second_value,operator);
    println!("The result is {} {} {} = {:.5}", first_value, operator, second_value, result);
}
