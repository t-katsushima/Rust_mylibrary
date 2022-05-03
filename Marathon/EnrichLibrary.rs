// cf. https://blog-dry.com/entry/2020/03/22/230803#Rust-で実装する-2

pub trait EnrichString<T> {
    fn hello(self);
}
impl EnrichString<String> for String {
    fn hello(self) {
        println!("hello!");
    }
}
