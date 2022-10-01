use std::fs::File;
//I added a test.txt file 
fn main(){
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error)=>{
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}