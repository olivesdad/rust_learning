pub fn calculate_mean(v: Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    let mut count = 0;
    for n in v {
        println!("number:{n}");
        sum += n;
        count += 1;
    }
 
    sum as f64 / count as f64
}
