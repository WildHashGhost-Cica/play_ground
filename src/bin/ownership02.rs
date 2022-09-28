fn re(v: Vec<i32>) -> Vec<i32>{
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>) {
   println!("{}", (*v)[10] + (*v)[12]);
   //println!("{}", &v[10] + &v[12]);
}

fn borrow2(v: &Vec<i32>){
    println!("{}", &v[10] + &v[11]);
}

fn borrow3(v: &Vec<i32>){
    println!("{}", &v[11] + &v[13]);
}

fn main(){
    let mut v = Vec::new();

    for i in 1..1000{
        v.push(i);
    }

    v = re(v);

   println!("Still own v: {} {}", v[0], v[1]);

   borrow1(&v);
   println!("Still own v: {} {}", v[0], v[1]);

   borrow2(&v);
   println!("Still own v: {} {}", v[0], v[1]);

   borrow3(&v);
   println!("Still own v: {} {}", v[0], v[1]);
    
}