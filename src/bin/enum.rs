//if we don't want warn messages 
#![allow(dead_code)]

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Right(Point),
    Left(Point),
}

#[derive(Debug)]
enum Keys{
    UpKey(String),
    DownKey(String),
    RightKey(String),
    LeftKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed a")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String{
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main(){
    let u = Direction::Up(Point {x: 0, y: 1});

    let k = u.match_direction();

    let x = k.destruct();

    println!("{:?}", k);
    println!("{:?}", x);

    let u = 10;
    let v = &u;
    let ref z = u;

    if z == v{
        println!("they are equal");
    }
}