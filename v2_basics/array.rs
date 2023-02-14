//array.rs
//
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for p in 0..4 {
        println!("[{}] = {}", p, arr[p]);
    }
    println!("length {}", arr.len());

    let res = sum(&arr);
    println!("sum {}", res);
}
