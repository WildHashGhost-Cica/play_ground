#[derive(Debug, Clone)]

struct A(i32);

struct B(f32);

fn main(){
    let a = A(32);
    let b = B(12.13);
    let c = a.clone() ;

    println!("{:?}", a);
}