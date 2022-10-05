
//fn f(i: i32) -> i32 { i + 1 }  this is equal with line #5 , let f = |i| i + 1;

fn main(){
    let f = |i| i + 1;
    let x = 10;
    f(x);
    println!("{}", f(x));

    let p = || println!("this is cosure");
    //run p
    p();
}