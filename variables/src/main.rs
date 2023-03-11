fn main() {
    let mut x = 5;
    println!("Значение х равно {}", x);
    x = 6;
    println!("Значение х равно {}", x);

    const MAX_POINT: u32 = 100_000;
    println!("max point = {}", MAX_POINT);

    println!("--------------------");
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x);

    println!("--------------------");
    let x = 2.1;
    let y: f32 = 3.2;
    println!("x = {}, y = {} ", x, y);

    println!("--------------------");
    let tup: (i32, f64, u8, ) = (500, 6.4, 3);
    let (_x, _y, _z) = tup;

    println!("{}", _y );
    println!("{}", tup.0 );
    println!("{}", tup.1 );
    println!("{}", tup.2 );

    println!("----------------------");
    let months = ["Январь", "Февраль", "Март", "Апрель", "Май"];
    println!("{}", months[0]);
    println!("{}", months[2]);
    let i = {
        9 + 4
    };
    println!("{}", second_function(i));
}

fn second_function(i: i32) -> &'static str {
    println!("second fun, arg = {}", i);
    "RET"
}