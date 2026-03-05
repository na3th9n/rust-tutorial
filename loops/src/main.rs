fn main() {
    // let mut counter = 0;
    
    // let result = loop {
    //     println!("Counter value: {counter}.");

    //     if counter == 10 {
    //         break counter * 2
    //     }

    //     counter += 1;
    // };

    // println!("Counter final value: {result}.")
    let mut count = 0;
    'counting_up: loop {
        println!("The current count is {count}");

        let mut inner_count = 0;

        loop {
            println!("The inner loop count is {inner_count}.");
            
            if inner_count == 2 {
                break 'counting_up;
            }


        }
    }
}
