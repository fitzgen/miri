struct Struct(i32);

trait Trait {
    fn method(&self);
}

impl Trait for Struct {
    fn method(&self) {
        assert_eq!(self.0, 42);
    }
}

fn main() {
    let y: &Trait = &Struct(42);
    y.method();
    /*
    let x: Box<Fn(i32) -> i32> = Box::new(|x| x * 2);
    assert_eq!(x(21), 42);
    let mut i = 5;
    {
        let mut x: Box<FnMut()> = Box::new(|| i *= 2);
        x(); x();
    }
    assert_eq!(i, 20);
    */
}
