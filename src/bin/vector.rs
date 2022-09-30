fn main(){
    let x = vec![1, 2, 3, 4];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{}", i);
    }

    println!("{:?} {:?}", x, v);
    println!("{:?} {} {}", &v, v.len(), v.capacity());

    let mut n: Vec<i32> = Vec::new();
    println!("{:?} {} {}",&n,n.len(),n.capacity());
    println!("{:?}", n.pop());
}