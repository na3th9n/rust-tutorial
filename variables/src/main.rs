// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // shadowing();
    tuple();
}

fn shadowing() {
    let x = 5;
    println!("The value of x is {x}");
    let x = x + 1;
    {
        let x = x + 6;
        println!("Inner scope x is {x}");
    }
    println!("The value of x is {x}");
}

fn tuple() {
    let tup: (i32, f64, u8) = (54, 32.5, 5);

    let (x, y, z) = tup; // destructering
    let z2 = tup.2;
    println!("The value of y is {y}");

    println!("The value of z is {z2}");
}