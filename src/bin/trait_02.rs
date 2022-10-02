#[derive(Debug, Clone, Copy)]

struct A(i32);
#[derive(Eq,PartialEq, PartialOrd, Ord)]
struct B(i32);

fn main(){
    let a = A(32);
    let b = B(12);
    //add &a if you want to use again a, or #[derive(Copy)]
    let c = a;

    println!("{:?}", a);
}