# CircularArray Documentation

Welcome to the `CircularArray` documentation. `CircularArray` is a Rust module that implements a circular array, allowing for infinite pushes into a fixed-size array. This document will guide you through using `CircularArray`, including how to manipulate array indices directly and how to convert the circular array into a standard fixed-size array.

## Features

- **Fixed Size**: Initialize with a predetermined size that cannot be exceeded.
- **Infinite Pushes**: Allows for an unlimited number of elements to be pushed into the array. When the array reaches its capacity, new elements overwrite the oldest ones.
- **Index Manipulation**: Provides the ability to directly manipulate elements at specific indices using both `Index` and `IndexMut` traits.
- **Conversion to Array**: Supports converting the current state of the circular array into a fixed-size array representation.

## Usage

To use `CircularArray`, include it in your Rust project and follow the examples below to understand how to interact with it.

### Index and IndexMut Example

This example demonstrates how to manipulate array elements using indices. It shows how to modify elements directly and verify their values using assertions.

```rust
#[test]
#[allow(non_snake_case)]
fn test_Index_and_IndexMut() {
    let mut arr = CircularArray::<3, u32>::new();
    arr.push(0);
    arr.push(0);
    arr.push(0);
    arr.push(0);
    arr.push(0);
    arr[0] = 1;
    arr[1] = 2;
    arr[2] = 3;
    assert_eq!(arr[0], 1);
    assert_eq!(arr[1], 2);
    assert_eq!(arr[2], 3);
}
```

### Conversion to Array

This example illustrates how to convert the state of the `CircularArray` into a fixed-size array. It also shows how the circular nature of the array works with the `push` method, where new elements replace the oldest ones in the array.

```rust
#[test]
fn test_to_array() {
    let mut arr = CircularArray::<3, u32>::new();
    arr.push(1);
    arr.push(2);
    arr.push(3);
    assert_eq!(arr.to_array(), [1, 2, 3]);
    arr.push(4);
    assert_eq!(arr.to_array(), [2, 3, 4]);
}
```

## Conclusion

`CircularArray` provides a powerful and flexible way to work with fixed-size arrays in Rust, especially when dealing with data streams that require overwriting old data with new entries. Through the use of indexing traits and conversion methods, it offers an intuitive API for managing and interacting with circular arrays.