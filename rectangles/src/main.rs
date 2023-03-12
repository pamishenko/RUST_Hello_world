#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn exist(&self, second: &Rectangle) -> bool{
        self.height > second.height && self.width > second.width ||
            self.height > second.width && self.width > second.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{height: size, width: size}
    }
}

fn main() {
    let rect1: Rectangle = Rectangle{width: 30, height: 50};
    let rect2: Rectangle = Rectangle{width: 3, height: 52};
    let rect3: Rectangle = Rectangle{width: 40, height: 20};
    let sqare = Rectangle::square(16);

    println!(
        "Площадь прямоугольника равна {} квадратным пикселам.",
        rect1.area()
    );
    println!("2 -> 1 {}", rect1.exist(&rect2));
    println!("3 -> 1 {}", rect1.exist(&rect3));
    println!("rect = {:?}", rect1);
    println!("square = {:#?}", sqare);
}
