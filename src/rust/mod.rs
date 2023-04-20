pub trait RustType {
    fn to_type() -> String {
        "test".to_string()
    }
    fn as_rust_type(&self) -> String {
        "test".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::RustType;

    struct Foo {
        _name: String,
        _age: i32,
    }

    impl RustType for Foo {}
    #[test]
    fn rust_types() {
        let t = Foo::to_type();
        let v: Foo = Foo {
            _name: "ab".into(),
            _age: 10,
        };
        dbg!(t);
        dbg!(v.as_rust_type());
    }
}
