struct Object {
    width: u32,
    height: u32,
}

fn area(obj: &Object) -> u32{
    obj.width * obj.height

}

fn main(){
    let o = Object{
        width: 35,
        height: 55,
    };

    println!("{} x {} with area: {} ", o.width, o.height, area(&o));
}