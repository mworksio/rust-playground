trait Cat  {
    fn Meow(&self);
}

struct Dog;
impl Cat for Dog {
    fn Meow(&self) {
       println!("meow from dog") 
    }
}

struct People;
impl Cat for People {
    fn Meow(&self) {
        println!("meow from people")
    }
}

fn say(c: & dyn Cat) {
    c.Meow();
}

fn said<T>(c: &T) where T:Cat {
    c.Meow();
}

fn main() {
    let p = People;
    say(&p);
    let d = Dog;
    said(&d);
}
