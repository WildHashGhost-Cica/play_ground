//we got first time error because lifetime wasn't specifier so we add 'a
fn pr<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() == y.len(){
        x
    }else{
        y
    }
}

fn main(){
    let a = "a string";
    let b = "b string";

    let c = pr(a, b);

    println!("{}", c);
}