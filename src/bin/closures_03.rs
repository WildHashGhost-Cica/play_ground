fn run<F>(f: F)
where F: Fn() {
    f();
}

fn add3<F>(f:F) -> i32
where F: Fn(i32) -> i32 {
    f(3)
}



fn main(){
    let p = || println!("hello from run function!");
    run(p);

    let x = |i| i * 10;

    println!("3 * 10 = {}", add3(x));

   
}