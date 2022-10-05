use std::collections::HashMap;

macro_rules! new_map{
    ($($key: expr => $val: expr)+) => {
        {
            let mut map = HashMap::new();

            $(
                map.insert($key,$val);
            )*
          map
        }
    };
}

fn main(){
    let m = new_map!{
        "one" => 1
        "two" => 2
        "three" => 3
    };
    println!("{:?}", m);
}