use self::types::FormatError;
use specta::{
    ts::{self, ExportConfiguration},
    NamedType, Type,
};

mod types;

/// Creates a type as a string
///
/// The type needs to implement [`specta::Type`], the easiest way to do this is
/// to add a `#[derive(Type)]` to your struct/enum
/// # Examples
/// ```
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
pub fn generate_typedoc<T: Type + NamedType>() -> Result<String, FormatError> {
    let export_str = ts::export::<T>(&ExportConfiguration::default()).unwrap();
    trim_export(&export_str)
}

fn trim_export(exported_str: &str) -> Result<String, FormatError> {
    match exported_str.starts_with("export") {
        true => Ok(exported_str.trim_start_matches("export ").to_string()),
        false => Err(FormatError::TrimError),
    }
}
