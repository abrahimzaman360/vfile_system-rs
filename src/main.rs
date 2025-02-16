#![allow(dead_code, unused)]

mod exercise;
mod project;

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

    // Structs:
    let mut vehicle_roof = exercise::structs::VehicleRoof::new(); // Create an empty VehicleRoof

    // Create a new vehicle
    let vehicle = exercise::structs::Vehicle {
        v_type: Some(exercise::structs::Type::CIVIC),
        v_num_plate: Some("ABC-123".to_string()),
        price: Some(25000.0),
        condition: Some(exercise::structs::Condition::NEW),
    };

    // Add the vehicle to VehicleRoof
    vehicle_roof.add_vehicle(vehicle);

    // Print the entire structure
    println!("{:#?}", vehicle_roof);

    // Strings & Slices:
    println!("{} ", exercise::strings::strings("Super Mocha"));
    // exercise::strings::super_slice();

    // Error Handling:
    // println!("{:#?}", exercise::error::error().unwrap());

    // Ownership Rules:
    exercise::ownership::user();

    // Test calculator:

    let num1 = 10;
    let op = '-';
    let num2 = 5;

    let some_val = project::calculator::calculator(num1, op, num2).unwrap();
}
