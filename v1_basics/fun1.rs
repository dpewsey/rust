//fun1.rs
// function types have to be defined
// f64 is float type
fn sqr(x: f64) -> f64 {
    return x * x;
}
// u64 is intergral type
fn ssqr(x: u64) -> u64 {
    x * x
}

// float into absolute
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x 
    } else {
        -x
    }
}

// clamps to force into value 12.0 would print as 10
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2 
    } else {
        x 
    }
}
// factorial
fn factor(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factor(n-1)
    }
}
// references
fn by_ref(x: &i32) -> i32{
    *x + 1
}
// modifies
fn modifies(x: &mut f64) {
    *x =1.0;
}
fn main () {
    let res = sqr(2.0);
            println!("square is {}", res);
    let ress = ssqr(2);
            println!("square is {}", ress);
    let res = abs(2.0);
        println!("test {}", res);
    let res = clamp(12.0,1.0,10.0);
        println!("clamppped {}", res);
    let res = factor(5);
        println!("factorial is {}", res);
    
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1,res2);


    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);


}
