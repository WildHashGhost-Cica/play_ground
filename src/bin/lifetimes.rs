fn main(){
    let x; //lifetime a
    {//lifetime b
        let y = 10; 
        x = &y;
    }
    //if we delete {} , the livetime will be the same and we don't get error
    println!("{}", x);
}