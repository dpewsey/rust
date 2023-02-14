// cirles.rs

//fn sqr(r: u64) -> u64 {
//    r * r
//}

fn area(r: f64, pi: f64) -> f64 {
    pi * r * r
}

fn circum(r: f64, pi: f64) -> f64 {
    2.0 * pi * r
}
fn main() {
    let pi: f64 = 3.14159;
    //    for i in 0..10 {
    //        let rad = sqr(i);
    //        println!("{}", rad);
    //    }

    for i in 0..10 {
        let i = f64::from(i) * 1.0;
        let area = area(i, pi);
        let circum = circum(i, pi);
        println!(
            "If Radius is {} then Area is {} and Circumference is {}",
            i, area, circum
        );
    }
}
