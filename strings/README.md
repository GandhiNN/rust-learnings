# Strings in Rust
Rust has more than one type of strings, namely `String` and `str`
Generally speaking, the difference comes down to ownership and memory.
What they have in common is that they are guaranteed to be valid UTF-8.

## `String`
It is an owned type that needs to be allocated. It has dynamic size and hence its size is unknown at compile time, since the capacity of the internal array can change at any time.

This type itself is a struct of the form
```
pub struct String {
    vec: Vec<u8>
}
```
Since it contains a `Vec`, it has a pointer to:
- A chunk of memory
- A size -> gives us the length of the string.
- A capacity -> gives information of how much long it can get before we need to reallocate.

`String` is very flexible in terms of usage. We can always create new, dynamic strings with it and mutate them. 
There's a cost that we need to always allocate new memory to create them.

## `&String`
It's simply a reference to a `String`. It is not an owned type and its size known at compile-time (it's only a pointer to an actual `String`)

Because it's not an owned type, we can pass it around, as long as the thing we are referencing does not go out of scope and we do not need to worry about allocations.

`&String` can be deref-coerced to `&str` by the Rust compiler, but it does not work the other way around.