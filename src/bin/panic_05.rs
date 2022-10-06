fn exit(x: Option<i32>) {
    let y = x.expect("we got none");
    if y == 0 {
        panic!("we got nothing");
    }
    println!("everything is fine!");
}

fn main(){
    exit(Some(1));
    exit(Some(10));
    exit(None);
    exit(Some(0));
}