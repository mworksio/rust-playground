struct Foo{
    name: String
}

#[derive(Default)]
struct FooBuilder {
    _name: String
}

#[async_trait::async_trait]
trait Printer {
    async fn print(self);
}

#[derive(Default)]
struct PrinterF {}

#[async_trait::async_trait]
impl Printer for PrinterF {
    async fn print(self){
        let f = FooBuilder::new().with_name("Jack".to_string()).build();
        println!("name: {}", f.name);
    }
}

impl FooBuilder {
    fn new() -> FooBuilder {
        // FooBuilder::default()
        FooBuilder{
            _name: "Leo".to_string(),
        }
    }
    fn with_name(mut self, name: String) -> FooBuilder {
        println!("set name");
        self._name = name;
        self
    }
    fn build(self) -> Foo {
        println!("build");
        Foo {
            name: self._name,
        }
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let p = PrinterF::default();
        p.print().await;
    });

    println!("hello");
}
