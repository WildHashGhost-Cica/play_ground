macro_rules! exame {
    ($l:expr; and $r:expr) => (
        println!("{:?} and {:?} is {:?}",
                stringify!($l),
                stringify!($r), 
                $l && $r)
    );

    ($l:expr; or $r:expr) => (
        println!("{:?} or {:?} is {:?}",
                stringify!($l),
                stringify!($r),
                $l || $r)
    );
}

fn main(){
    exame!(1 == 1; and 2 == 1 + 1 );
    exame!(true; or false);
}