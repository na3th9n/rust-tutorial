fn main() {
    let number = 3;

    if number < 5 {
        println!("The number is less than 5")
    } else {
        println!("The number is equal to or more than 5");
    }

    if_in_let_statement(false);
}

fn if_in_let_statement(bool_val: bool) {
    let x = if bool_val { 5 } else { 6 };
    
    println!("The value of if_in_let_statement is {x}");
}

