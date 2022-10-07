pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32{
    a +b
}

pub fn greeting(name: &str) -> String {
     format!("hello {}!", name)
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works(){
        assert_eq!(5, internal_adder(2,  2));
    }

    #[test]
    #[should_panic]
    fn another(){
        assert!(true == false);
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}