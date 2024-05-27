//! # cw-into-event
//! A derive macro make a sturct able to pass to functions like
//! [`cosmwasm_std::Response::add_event`](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/struct.Response.html#method.add_event).
//!
//! ## Description
//! [`IntoEvent`] is a derive macro implements `Into<cosmwasm_std::Event>`.
//!
//! This means structs with `#[derive(IntoEvent)]` can be passed to [`cosmwasm_std::Response::add_event`](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/struct.Response.html#method.add_event) and other functions takes `impl Into<cosmwasm_std::Event>`.
//!
//! See [`IntoEvent`] for more details.
//!
//! ## Usage
//! Add `#[derive(IntoEvent)]` to your struct.
//!
//! ```
//! use cw_into_event::IntoEvent;
//!
//! #[derive(IntoEvent)]
//! struct MyStruct {
//!     string: String,
//!     // `num.to_string()` is used to make the attribute's value
//!     #[use_to_string]
//!     num: u32,
//!     // `String::from(address)` is used to make the attribute's value
//!     #[to_string_fn(String::from)]
//!     address: cosmwasm_std::Addr,
//! }
//!
//! // test `MyStruct::into<cosmwasm_std::Event>`
//! let my_struct = MyStruct {
//!     string: "foo".to_string(),
//!     num: 42,
//!     address: cosmwasm_std::Addr::unchecked("bar"),
//! };
//! let my_event: cosmwasm_std::Event = my_struct.into();
//! let expected = cosmwasm_std::Event::new("my_struct")
//!     .add_attribute("string", "foo")
//!     .add_attribute("num", "42")
//!     .add_attribute("address", "bar");
//! assert_eq!(expected, my_event);
//! ```
#[macro_use]
extern crate syn;

mod into_event;

use proc_macro::TokenStream;

/// generate an ast for `impl Into<cosmwasm::Event>` from a struct
///
/// Structure:
///
/// ```ignore
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
/// ```ignore
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
