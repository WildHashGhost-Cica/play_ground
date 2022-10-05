macro_rules! calc{
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    (eval $e:expr, $(eval $es:expr), +) => {
        {
            calc!{eval $e}
            calc!{$(eval $es),+}
        }
    };
}

fn main(){
    calc! {
        eval 4 * 5,
        eval 4 + 10,
        eval (10*3)-20
    };
}