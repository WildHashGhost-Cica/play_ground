use std::ops::Mul;

trait Shape<T> {
    fn area(&self) -> T;
}

struct Rectangle<T: Mul> {
    x: T,
    y: T,
}



impl <T: Mul<Output = T> + Copy> Shape<T> for Rectangle<T>
    {
        fn area(&self) -> T{
            self.x * self.y
        }
    }

    

fn main(){
     let r = Rectangle { x: 10, y: 20};
     let r2 = Rectangle {x: 10.10, y: 20.31};
    

     println!("{} {}", r.area(), r2.area());
}