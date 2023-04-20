use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum FormatError {
    TrimError,
}

#[derive(Debug)]
pub struct FormattedType(pub String);

impl Display for FormattedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Trait implementing trim functions that you can use on a
/// [`FormattedType`].
///
/// You can use multiple functions on a single [`FormattedType`] like so:
///
/// ```
/// use crate::generate_typedoc;
/// use crate::TrimFunctions;
/// use specta::Type;
///
/// #[derive(Type)]
/// struct Foo {
///     bar: String,
///     baz: i32
/// }
/// let data = generate_typedoc::<Foo>().unwrap();
/// let trimmed_a = data.trim_export().to_string();
/// assert_eq!(trimmed_a, "type Foo = { bar: string, baz: number}");
/// let trimmed_b = data.trim_export().replace_equals().remove_type_word().to_string();
/// assert_eq!(trimmed_b, "Foo { bar: string, baz: number }")
/// ```
pub trait TrimFunctions
where
    Self: Display + From<String>,
{
    fn trim_export(&self) -> Self {
        Self::from(self.to_string().replace("export ", ""))
    }
    fn remove_type_word(&self) -> Self {
        Self::from(self.to_string().replace("type ", ""))
    }
    fn replace_equals(&self) -> Self {
        Self::from(self.to_string().replace(" =", ""))
    }
}

impl From<String> for FormattedType {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TrimFunctions for FormattedType {}
