/*
* Program to take input from the user of integers and find the mean
* usage will be
* 1. Run the program
* 2. Input integers for as long as they want 1 at a time
* 3. Quit when a non integer is entered
*/

// Use stuff

mod calculator;
mod numbers;

fn main() {
    //get the vector of ints from user
    let numbers = numbers::get_numbers();

    //calculate mean
    let mean: f64 = calculator::calculate_mean(numbers);

    //print answer
    println!("mean is: {}", mean);
}

