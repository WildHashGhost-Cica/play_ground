fn main() {
    use std::mem;

    let t = (1, 'a', false);
    let f = (2, t);
    let m = (2,(1, 'a', false));
    //tuples you should give wich element want to print
    println!("{}", t.0);
    println!("{} {}", t.0, t.1);
    println!("{} {} {}", t.0, t.1, t.2);
    println!("{:?}", f);
    println!("{:?}",m);
    println!("{:#?}", m);
    
    let xs = [4, 5, 6, 7 ,8];
    println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));

    //slice of array
    let ys = &xs[2..4];
    println!("{:?}", ys);
    println!("{:?} {:?}", xs, ys);

    let s = "String".to_string();
    let ss= String::from("String");
    let slice = &ss[0..4];
    println!("{}", slice);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let u = h + &w;
    println!("{}",u);
    //empty tuple
    let k = ();


}