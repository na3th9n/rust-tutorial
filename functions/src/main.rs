fn main() {
    labeled_parameter(5, 'm');
    let x = five();
    println!("The value of x is {x}");
}

fn labeled_parameter(x: i32, unit_label: char) {
    println!("The value of x is {x}{unit_label}");
}

fn five() -> i32 {
    5
}