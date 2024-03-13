# cw-into-event

cw-into-event provides the `IntoEvent` macro.
`IntoEvent` is a derive macro and it automatically implements
`Into<cosmwasm_std::Event>` to a struct.

## Example

### Cargo.toml

```toml
# ... #

[dependencies]
# ... #
cosmwasm-std = "2.0.0"
cw-into-event = "0.1.0"
# ... #

# ... #
```

### your contract

```rust
#[derive(IntoEvent)]
struct TransferEvent {
    from: Addr,
    receiver: Addr,
}

let transfer_event = TransferEvent {
    from: Addr::unchecked("alice"),
    receiver: Addr::unchecked("bob"),
};

// Before cosmwasm merged https://github.com/CosmWasm/cosmwasm/pull/2044
let response = Response::<Empty>::new().add_event(transfer_event.into());

// After cosmwasm merged https://github.com/CosmWasm/cosmwasm/pull/2044,
// `.into()` is not needed and you can write `response` simply
let response = Response::<Empty>::new().add_event(transfer_event);
```

## Usage
### Struct

```rust
#[derive(IntoEvent)]
struct MyStruct {
    field1: type1,
    // if the value's type does not implement `Into<String>` trait
    // and it implements `ToString` trait, programmers can specify
    // to use `field1.to_string()` to get string
    // by applying `use_to_string`.
    #[use_to_string]
    field2: type2,
    // if the value's type does not implement both `Into<String>` and
    // `ToString` traits, programmers need specify a function
    // to get string with `casting_fn(field3)` by applying
    // `to_string_fn(casting_fn)` attribute.
    // this `casting_fn` needs to have the type `type3 -> String`.
    #[to_string_fn(casting_fn)]
    field3: type3,
}
```

### Automatically generated implements

```rust
impl Into<cosmwasm_std::Event> for MyStruct {
    fn into(self) -> cosmwasm_std::Event {
        cosmwasm_std::Event::new("my_struct")
            .add_attribute("field1", self.field1)
            .add_attribute("field2", self.field2.to_string())
            .add_attribute("field3", casting_fn(self.field3))
    }
}
```
