use std::sync::{Mutex,Arc};
use std::thread;

fn main(){
    let c = Arc::new(Mutex::new(0));
    let mut hs = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&c);
        let h = thread::spawn(move || {
            let mut num = c.lock().unwrap();

            *num += 1;
            //will print out number between 1-10
            println!("{}", num);
        });
        hs.push(h);
    }
    for h in hs{
        h.join().unwrap();
    }
    println!("Result:{}", *c.lock().unwrap());
}