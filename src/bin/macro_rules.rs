macro_rules! a_macro {
    () => (
        println!("this is a macro");
    )
}

fn main(){
    a_macro!();
}