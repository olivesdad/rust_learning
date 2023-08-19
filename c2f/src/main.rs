use std::io;


fn main() {

    let mut buf = String::new();
    println!("Enter a temp in Celsius");
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    let c:f32 = buf.trim().parse().expect("Failed to Parse. Enter a number");

    println!("{c} Celsius is = {} Farenheit", 
                conver_to_farenheit(c)); 

}

fn conver_to_farenheit( c:f32) -> f32 {
    (c * 9.0/5.0) + 32.0
}
