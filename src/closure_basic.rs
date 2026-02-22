fn run(f: fn(i32) -> i32) -> i32 {
    f(100)
}

fn main() {
    let _my_closure_fn = |x: i32| x * 2;
    fn my_traditional_fn(x: i32) -> i32 {
        x * 2
    }
    let result = run(my_traditional_fn);
    println!("{result}")
}
