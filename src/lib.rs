#[macro_use]
extern crate syn;

mod into_event;

use proc_macro::TokenStream;

/// generate an ast for `impl Into<cosmwasm::Event>` from a struct
///
/// Structure:
///
/// ```no_test
/// #[derive(IntoEvent)]
/// struct MyStruct {
///     field1: type1,
///     // if the value's type does not implement `Into<String>` trait
///     // and it implements `ToString` trait, programmers can specify
///     // to use `field1.to_string()` to get string
///     // by applying `use_to_string`.
///     #[use_to_string]
///     field2: type2,
///     // if the value's type does not implement both `Into<String>` and
///     // `ToString` traits, programmers need specify a function
///     // to get string with `casting_fn(field3)` by applying
///     // `to_string_fn(casting_fn)` attribute.
///     // this `casting_fn` needs to have the type `type3 -> String`.
///     #[to_string_fn(casting_fn)]
///     field3: type3,
/// }
/// ```
///
/// Output AST:
///
/// ```no_test
/// impl Into<cosmwasm_std::Event> for MyStruct {
///     fn into(self) -> cosmwasm_std::Event {
///         cosmwasm_std::Event::new("my_struct")
///             .add_attribute("field1", self.field1)
///             .add_attribute("field2", self.field2.to_string())
///             .add_attribute("field3", casting_fn(self.field3))
///     }
/// }
/// ```
#[proc_macro_derive(IntoEvent, attributes(to_string_fn, use_to_string))]
pub fn derive_into_event(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    into_event::derive_into_event(derive_input)
}
