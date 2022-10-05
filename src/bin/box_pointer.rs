fn create() -> Box<dyn Fn()> {
    Box::new(move || println!("this is a closure in a box!"))
}

fn main(){
    let x = create();
    x();
}