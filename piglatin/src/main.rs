/*
* Piglatin Translator
* Ask user for a sentence then return it translated to piglatin
*/
use std::{thread, time};


fn main() {
    // Get sentance function
    let s_in = get_input();
    println!("Thinking....");
    let dur = time::Duration::from_millis(3000);
    thread::sleep(dur);
    println!("{s_in}");

    // Translate fn

    // Print results
}

fn get_input() -> String {

    use std::io;
    let mut buf = String::new();

    println!("What would you like me to translate?");

    match io::stdin().read_line(&mut buf) {
        Ok(b)  => {
            if b <= 1 {
                return String::from("NOTHING")
            } else {
                return buf;
            }
        },
        Err(_e) =>{
            return String::from("You didn't enter anything good");
        }
    }
}