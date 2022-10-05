struct A<'a, 'b>{
    x: &'a str,
    y: &'b str,
}
fn ab<'a, 'b>(x:&'a i32, y:&'b i32){}

fn a<'a>(s:&'a str) -> &'a str{
    s
}

impl <'a, 'b> A<'a, 'b> {
    fn slf(&self) -> &str{
        self.x
    }
}


fn main(){
    let a = A{x: "Hello", y: "there"};

   

}