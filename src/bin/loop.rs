fn main(){
   /* 'a:loop {
        println!("loop a");
        'b:loop {
            println!("loop b");
            'c:loop{
                println!("loop c");

                break 'b
            }
        }
        continue 'a
    }
*/
    let x = loop {
        break 10;
    };
    println!("x: {}", x)
}