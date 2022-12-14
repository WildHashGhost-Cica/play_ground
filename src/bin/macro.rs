macro_rules! x_and_y {
    (x => $e:expr) => (println!("X: {}", $e));
    (y => $e:expr) => (println!("Y: {}", $e));
}

macro_rules! build_fn {
    ($func_name:ident) => (
        fn $func_name(){
            println!("You called {:?}()", stringify!($func_name));
        }
    )
}

macro_rules! print_ex {
    ($e:expr) => (
        println!("{:?} = {:?}", stringify!($e),$e);
    )
}

fn main(){
    x_and_y!(x => 10);
    x_and_y!(y => 20 + 30);

    build_fn!(Hi_there);
    Hi_there();

    print_ex!({
        let y = 20;
        let z = 30;
        z + y +10 *3 *100
    });
}