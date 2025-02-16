#![allow(dead_code, unused)]
mod exercise;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Point3D(f32, f32, f32);

#[derive(Debug)]
struct Cirlce {
    center: Point,
    radius: u32,
}

#[derive(Debug, PartialEq)]
enum COLOR {
    RED,
    GREEN,
    BLUE,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    HSL { h: u8, s: u8, l: u8 },
}

fn main() {
    let red_color: COLOR = COLOR::RED;
    let green_color = COLOR::GREEN;
    let blue_color = COLOR::BLUE;

    // RGBA
    let rgba_color = COLOR::Rgba(10, 15, 12, 0.5);
    let color = COLOR::Hex("#fff".to_string());
    let hsl_color = COLOR::HSL {
        h: 10,
        s: 12,
        l: 15,
    };

    println!("{:?}", rgba_color);
    println!("{:?}", color);
    println!("Is Equal: {}", green_color == red_color);
    println!("Is Equal: {}", green_color == blue_color);
    println!("Is Equal: {:?}", hsl_color);

    // Optional Enum
    let super_optional: Option<i32> = None;
    let yuper: Option<i32> = Some(11);

    println!("{:?}", yuper);
    println!("{:?}", super_optional);

    // Optional
    let ok_mocha: Result<u32, String> = Ok(10);
    let err_mocha: Result<u32, String> = Err("Snake Life".to_string());

    println!("{:?} {:?}", ok_mocha, err_mocha);

    // Structs:
    let point = Point {
        x: 10 as f32,
        y: 12 as f32,
    };
    let new_struct = Cirlce {
        center: point,
        radius: 12,
    };

    println!(
        "x: {:?} y: {}, r: {}",
        new_struct.center.x, new_struct.center.y, new_struct.radius
    );

    // Some Shortcut:
    let x = 10 as f32;
    let y = 12 as f32;

    let p2 = Point { x, y };
    let p3 = Point { ..p2 };
    println!("P2: {:#?} P3: {:#?}", p2, p3);

    // Array:
    exercise::arrays::arrays();
}
