struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn area(&self) -> u32{
        self.width * self.height
    
    }

    fn new(width: u32, height: u32) -> Object{
        Object{
            width,
            height,
        }
    }
}
//we cope and add to impl the function and we change object to self
/*fn area(obj: &Object) -> u32{
    obj.width * obj.height

}*/


fn main(){
    let o = Object{
        width: 35,
        height: 55,
    };

    let obj = Object::new(57, 83);

    println!("{} x {} with area: {} ", o.width, o.height, o.area());
    println!("{} x {} with area: {} ", obj.width, obj.height, obj.area());

    

}