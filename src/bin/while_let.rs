fn main(){
    let mut n = Some(0);

    //replace loop wiht while let 
    /*
    loop {
        match n {
            Some(i) => if i >19 {
                println!("Quit");
                n = None;
            }else{
                println!("{}", i);
                n = Some(i +2);
            },
            _ => {
                break;
            }
        }
    }*/

    while let Some(i) = n {
        if i > 19 {
            println!("Quit");
            n = None;
        }else {
            println!("{}",i);
            n = Some(i+2);
        }
    }
}