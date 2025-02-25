# Simple HashMap Implementation

This is a simple implementation of a HashMap that behaves similarly to the native
HashMap in Rust.

It was generted, with minimal changes, by ChatGPT-4o on Feb. 23, 2025.

## Usage

To use this implementation, you can create a new HashMap with a specified capacity.

```rust
let mut map = HashMap::new(10);
```

The integer argument is the initial capacity of the map.

This minimal implementation only supports the `.insert()`, `.get()`, and `.remove()` methods.

To add key-value pairs to the map, use the `.insert()` method.

```rust
map.insert("key", "value");
```

To get a value from the map, use the `get` method.

```rust
let value = map.get("key");
```

To remove a key-value pair from the map, use the `remove` method.

```rust
map.remove("key");
```


