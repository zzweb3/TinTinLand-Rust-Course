trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
struct Circle {
    radius: f64,
}
impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * 3.14 * self.radius
    }
}

#[test]
fn test() {
    let circle = Circle::new(5.0);
    let shape: &dyn Shape = &circle;
   // let x = Box::new(circle);
    //println!("{}", x.area());

    println!("shape => {}", shape.area());
}