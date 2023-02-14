// add1.rs 
fn main() {
    // mut makes variable mutable
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);

    // f64 defines as a float
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64; 
    }
    println!("sum is {}", sum); 
}
