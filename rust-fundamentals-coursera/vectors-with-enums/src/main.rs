enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle(4.0, 6.0),
    ];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => base * height / 2.0,
        })
        .sum();

    let max_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => base * height / 2.0,
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("Total area: {:.2} sq. units", total_area);
    println!("Max area: {:.2} sq. units", max_area);
}
