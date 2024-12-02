use std::env::{args, Args};
fn main() {
    let mut args: Args = args();
    let wert1 = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let wert2 = args.nth(0).unwrap();
    let first_number =wert1.parse::<f32>().unwrap();
    let second_number =wert2.parse::<f32>().unwrap();
    let result = operate(operator,first_number,second_number);
    println!("{}",output(first_number, operator, second_number, result));
}
fn operate(operator: char, first_number: f32, second_number: f32)->f32{
    /*if operator == '+'{
        first_number + second_number
    }
    else if operator == '*'{
        first_number * second_number
    }
    else if operator == '-'{
        first_number - second_number
    }
    else if operator == '/'{
        first_number / second_number
    }
    else{0.0}*/
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _   => panic!("Invalid Operator used.")
    }

}
fn output(first_number:f32,operator:char,second_number: f32,result:f32)->String{
    format!("{} {} {} = {}", first_number,operator,second_number,result)
}
