struct Rectangle {
    width: u32,
    length:u32,
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        length: 50,
    };

    println!("Area is {}\n", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.length;
}
