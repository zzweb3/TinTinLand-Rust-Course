trait MyTrait {
    fn my_method(&self) -> i32;
}

struct MyStruct(i32);
impl MyTrait for MyStruct {
    fn my_method(&self) -> i32 {
        self.0
    }
}
trait MyOtherTrait {
    fn my_other_method(&self) -> i32;
}

// 💡 Implement MyOtherTrait for each type that implements MyTrait
impl<T: MyTrait> MyOtherTrait for T {
    fn my_other_method(&self) -> i32 {
        println!("my_struct -> myTrait, MyOtherTrait");
        self.my_method() * 2
    }
}

struct HeStruct;
impl MyTrait for HeStruct {
    fn my_method(&self) -> i32 {
        100
    }
}

#[test]
fn test() {
    let my_struct = MyStruct(10);
    println!("my: {}", my_struct.my_other_method());

    let he_struct = HeStruct;
    println!("he: {}", he_struct.my_other_method())
}