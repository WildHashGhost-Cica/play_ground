fn take(v: Vec<i32>){
    println!("We took v: {}", v[10] + v[100]);
}

fn cop(a: i32, b:i32){
    println!("{}", a + b);
}

fn main(){
    let s = String::from("String");
    let y = s;

    println!("{}", y);
    // or we should add & to bored 

    let x = String::from("Hello");
    let a = &x;

    println!("{}", x);
    println!("{}", &x);
    

    let mut v = Vec::new();
    
    for i in 1..1000 {
        
        v.push(i);
    }
    take(v);
    println!("Finished!");


    let a = 32;
    let b = 45;

    cop(a,b);

    println!("we have a: {} and b: {}", a, b);
}