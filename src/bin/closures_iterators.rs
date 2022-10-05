fn main(){
     let v = vec![1, 2, 3];

     println!("v {}", v.iter().any(|&x| x!= 2));

     for i in v.iter(){
        println!("{}", i);
     }
}