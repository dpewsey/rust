// for1.rs
fn main() {
    // not inclusive so prints 0-3( 4 numbers)
    for i in 0..4 {
        println!("Hello {}", i);
    }

    // if $i can be divided by 2 print even
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
    // does the same thing     
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
}
