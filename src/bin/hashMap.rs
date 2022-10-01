use std::collections::HashMap;

fn main(){
    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("string"), 49);
    //remove string
    hm.remove(&String::from("string"));

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    match hm.get(&String::from("random")){
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }
}