#!allow(unused)

// Key value pair
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32, f32, f32);

struct Empty;

// Nested struct
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}
fn main() {
    // Create
    let p = Point { x: 1.2, y: 2.1 };
    println!("Point x is {}, Point y is {}", p.x, p.y);

    let p = Point3d(1.0, 2.0, 3.0);
    println!("point3d: {}, {}, {}", p.0, p.1, p.2);

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 1.5, y: 2.0 },
        radius: 1,
    };
    // Debug
    // Read
    println!("circle: {:?}", circle);
    // Shortcut
    let x = 1.0;
    let y = 1.0;

    let p = Point { x, y };
    // Copy fields
    let p0 = Point { x: 1.0, y: 2.0 };
    let p1 = Point { x: 2.0, ..p0 };
    println!("p1: {:?}", p1);

    // Update
    let mut p = Point { x: 0.0, y: 0.2 };
    p.x += 1.2;
    p.y += 2.6;
    println!("{:?}", p);
}
