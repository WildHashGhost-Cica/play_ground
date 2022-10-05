use std::fs::File;

fn main(){
    let f = File::open("text.txt");

    let f = match f{
        Ok(file) => file,
        Err(e) => {
            panic!("{}", e);
        }
    };
}