use std::io;
pub fn get_numbers() -> Vec<i32> {
    let mut buf = String::new();
    let mut numbers: Vec<i32> = Vec::new();

    'read_input: loop {
        println!("Enter an integer (enter non integer to end): ");
        match io::stdin().read_line(&mut buf) {
            Ok(_i) => {
                let n = buf.trim().parse::<i32>();
                match n {
                    Ok(x) => {
                        numbers.push(x);
                        buf.clear();
                    }
                    Err(_e) => {
                        break 'read_input;
                    }
                }
            }
            Err(_e) => {
                break 'read_input;
            }
        }
    }
    numbers
}