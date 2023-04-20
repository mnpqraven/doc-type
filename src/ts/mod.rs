use self::types::FormatError;
use crate::ts::types::FormattedType;
use specta::ts::ExportConfiguration;
use specta::{NamedType, Type};

pub mod types;

/// Creates a String formatted to the target language
///
/// The type needs to implement [`specta::Type`], the easiest way to do this is
/// to add a `#[derive(Type)]` to your struct/enum
/// # Examples
/// ```
///  use specta::Type;
///
///  #[derive(Type)]
///  struct Foo {
///      name: String,
///      age: u32,
///  }
///  assert_eq!(
///      generate_typedoc::<Foo>(),
///      Ok("type Foo = { name: string; age: number }".to_string())
///  )
/// ```
pub fn generate_typedoc<T: Type + NamedType>() -> Result<FormattedType, FormatError> {
    let export_str = specta::ts::export::<T>(&ExportConfiguration::default()).unwrap();
    Ok(FormattedType(export_str))
}

#[cfg(test)]
mod tests {
    use crate::{generate_typedoc, ts::types::TrimFunctions};
    use specta::Type;

    #[derive(Type)]
    struct Tuple(i32);

    #[derive(Type)]
    struct Normal {
        _id: i32,
        _name: String,
    }

    #[test]
    fn tuple() {
        let a = generate_typedoc::<Normal>().unwrap();
        let t = generate_typedoc::<Tuple>().unwrap();
        dbg!(&a);
        dbg!(&a.trim_export());
        dbg!(&a.trim_export().replace_equals());
        dbg!(&a.trim_export().replace_equals().remove_type_word());
        dbg!(t);
    }
}
