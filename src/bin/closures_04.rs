fn run<F>(f:F)
where F: Fn(){
    f();
}

fn pr(){
    println!("this is normal function!");
}

fn main(){
    let p = || println!("hello from run function!");
    run(p);
    run(pr);
}