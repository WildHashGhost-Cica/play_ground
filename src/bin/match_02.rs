

fn main(){
    let n = 15;
    
    match n {
        1 => println!("ONE"),
        2|3|5|7|11 => println!("This is prime"),
        13|15|17|19 => println!("it's a teen"),
        _ => println!("Ain't special"),
    }

    let pair = (0, -2);
    
    
    match pair{
        (0, y) => println!("y:{}",y),
        (x, 0) => println!("x:{}",x),
        _ => println!("No match"),
    }

    let newpair = (5, -5);
    
    match newpair{
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal Zero"),
        (x, _) if x%2 == 0 => println!("x is even"),
        _ => println!("no match"),
    }

    let p = 5;

    match p {
        o @ 1..=12 => println!("n: {}", o),
        o @ 13..=19 => println!("n: {}", o),
        _ => println!("no match")
    }


    
    
}