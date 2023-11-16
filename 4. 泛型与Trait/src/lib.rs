
#[cfg(test)] 
mod tests {

    use super::*;


    use std::collections::HashMap;

    trait Shape {
        fn area(&self) -> f32;
    }

    struct Circle {
        radius: f32,
    }
    
    impl Shape for Circle {
        fn area(&self) -> f32 {
            3.14 * self.radius * self.radius
        }
    }

    struct Rectangle {
        x: f32,
        y: f32,
    }
    
    impl Shape for Rectangle {
        fn area(&self) -> f32 {
            0.5 * (self.x + self.y)
        }
    }

    
    #[test]
    fn test() {
        let mut shape_map: HashMap<&str, Box<dyn Shape>> = HashMap::new();

        shape_map.insert("circle", Box::new(Circle { radius: 2.0 }));
        shape_map.insert("rectangle", Box::new(Rectangle {x: 3.0, y: 4.0 }));

        let circle = shape_map.get("circle").unwrap();
        println!("circle = {}", circle.area());

        let rectangle = shape_map.get("rectangle").unwrap();
        println!("rectangle = {}", rectangle.area());
    }

}
