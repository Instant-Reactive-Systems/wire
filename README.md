# wire
A facade and utility crate for on-the-wire communication.

Provides `Req`, `Res` and `Error` as a generic API for req-res communication.
Also provides a utility macro for implementing actions and events for use with communication and also [`bevy`].

# Example
```rust
#[derive(wire::WireObj)]
#[rustfmt::ignore] // rustfmt will mess up the generated code
#[derive(Debug, Clone, PartialEq)]
pub enum MyEnum {
    Foo { a: u32, b: u32 },
    Bar(u32, u32),
}

fn main() {
    let a = A { a: 0xa, b: "b".to_string() };
    let b = B { horse: 42 };
}
```

[`bevy`]: https://bevyengine.org

# Adding as a dependency

```toml
[dependencies]
wire = { git = "ssh://git@github.com/Instant-Reactive-Systems/wire.git" }
```
