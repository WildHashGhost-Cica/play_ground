

fn main(){
    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("True");
    }
}