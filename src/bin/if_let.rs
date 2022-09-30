fn main(){
    let s = Some('c');

    /*
    //replace with if let 
    match s {
        Some(i) => println!("{}", i),
        _ => {}
    }
    */
    if let Some(i) = s {
        println!("{}", i);
    }else{
        {}
    }

    
}