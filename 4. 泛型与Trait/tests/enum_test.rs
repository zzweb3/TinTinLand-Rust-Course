trait Greeting {
    fn greeting(&self) -> &str;
}

struct Cat;
impl Greeting for Cat {
    fn greeting(&self) -> &str {
        "Meow!"
    }
}

struct Dog;
impl Greeting for Dog {
    fn greeting(&self) -> &str {
        "Woof!"
    }
}

fn print_greeting_static<G: Greeting>(g: G) {
    println!("{}", g.greeting());
}
fn print_greeting_dynamic(g: Box<dyn Greeting>) {
    println!("{}", g.greeting());
}

fn main() {
    print_greeting_static(Cat);
    print_greeting_static(Dog);
    
    print_greeting_dynamic(Box::new(Cat));
    print_greeting_dynamic(Box::new(Dog));
}
