/// 声明一个图形 shape trait
trait Shape {
    /// 计算面积
    fn area(&self) -> f64;

    /// 计算周长
    fn perimeter(&self) -> f64;
}

/// 圆形结构体
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        return Circle { radius };
    }
}

/// 为 Circle 实现 Shape
impl Shape for Circle {
    fn area(&self) -> f64 {
        return 3.14 * self.radius * self.radius;
    }

    fn perimeter(&self) -> f64 {
        return 2.0 * 3.14 * self.radius;
    }
}

#[test]
fn test() {
    let circle = Circle::new(5.0);

    let shape: &dyn Shape = &circle;

    println!("shape => {}", shape.area());
}