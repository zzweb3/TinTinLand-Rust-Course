trait Animal {
    fn talk(&self);
}

#[derive(Debug)]
struct Cat;
impl Animal for Cat {
    fn talk(&self) {
        println!("Meow!");
    }
}

#[derive(Debug)]
struct Dog;
impl Animal for Dog {
    fn talk(&self) {
        println!("Bark!");
    }
}

fn animal_talk(a: Box<dyn Animal>) {
    a.talk();
}


#[test]
fn test() {
    let c = Cat{};
    let d = Dog{};

    animal_talk(Box::new(c));
    animal_talk(Box::new(d));

    println!("{:#?}", c);
    println!("{:#?}", d);

}
