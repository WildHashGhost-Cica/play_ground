#[cfg(target_os = "linux")]
fn are_you_on_linux(){
    println!("running linux!");
}

#[cfg(not(target_os =  "linux"))]
fn are_you_on_linux(){
    println!("not running linux");
}

fn main(){
    are_you_on_linux();

    println!("check os again");
    if cfg!(target_os = "linux"){
        println!("definitely linux");
    }else{
        println!("not linux");
    }
}