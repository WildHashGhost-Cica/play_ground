macro_rules! compr{
        ($id1:ident | $id2:ident <- [$start:expr; $end: expr] , $cond: expr) => {
            {
                let mut vec = Vec::new();

                for num in $start..$end + 1 {
                    if $cond(num) {
                        vec.push(num);
                    }
                }
                vec
            }
        };
}

fn even(x: i32) -> bool {
    x%2 == 0
}

fn odd(x: i32) -> bool {
    x%2 != 0
}

fn main(){
    let evens = compr![x|x <- [1;10],even];
    println!("{:?}", evens);

    let odds = compr![y|Y <- [1;10], odd];
    println!("{:?}", odds);
}