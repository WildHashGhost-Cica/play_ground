fn count(v: &Vec<i32>, val:i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}

fn main() {
    let v = vec![1,1,1,2,2,3,3,3,4,5,5,5,5,8,8,7,7,9];
    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {} times", i, r);
    }
    println!("{} {}", v[0], v[2]);
}