fn exit(x: i32) {
    if x ==0 {
        panic!("we got a 0");
    }
    println!("things are fine!");
}

fn main(){
    exit(1);
    exit(0);
}