#[link(name = "math_ex3")]
unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 10;
    let b = 5;

    let res = unsafe {
        add(a, b)
    };
        
    println!("I love C because it's the best way to understand the metal!");
    println!("And I love Rust because it's the best way to not crash these one! And because it's also beautyful");
    println!("{} + {} = {} -> Calculated in C !!! ", a, b, res);

}