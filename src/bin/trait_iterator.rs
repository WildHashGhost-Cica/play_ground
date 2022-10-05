trait Iterator{
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn main(){
    let v = vec![1, 2, 3];
    v.iter().next();
}