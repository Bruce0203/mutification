This crate may cause Undefined Behavior

# mutification
Convert immutable reference to mutable reference.
When creating a getter and setter as a trait to get a field from a structure in a complex gaming system, this is a crate that safely changes an immutable reference to a mutable reference so that you can bypass the Rust compiler rules where you can't borrow another field at the same time!

- `mutification` crate is more conventient than `to-mut`/`to-mut-proc-macro`.
- Support generics.


**example**
```rust
use mutification::ToMut;

#[derive(ToMut)]
struct Player {
    name: &'static str
}
fn test_to_mut(player: &Player) {
    player.to_mut().name = "Bruce";
}
```
