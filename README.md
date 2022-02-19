## References

Rust references come in two flavors:

`&T`
* immutable, shared reference
* modifying the value they point to is forbidden
* similar to `const T*` in C

`&mut T`
* mutable, exclusive reference
* you may not have other references to that value
* similar to `T*` in C

"single writer or multiple readers"

## Raw Pointers

`*mut T` and `*const T`

Unsafe!

## Arrays, Vectors, and Slices

`[T; N]`
* an array of N values, each of type T
* can't append new elements or shrink an array

`Vec<T>`
* dynamically allocated, growable sequence of values of type T
* lives in heap

`&[T]` and `&mut [T]`
* shared slice of Ts and mutable slice of Ts

## Strings

___ | `Vec<T>` | `String`
:---|:---:|:---:
Automatically free buffers | Yes | Yes
Growable | Yes | Yes
`::new()` and `::with_capacity()` type-associated functions | Yes | Yes
`.reserve()` and `.capacity()` methods | Yes | Yes
`.push()` and  `.pop()` methods | Yes | Yes
Range syntax `v[start..stop]` | Yes, returns `&[T]` | Yes, returns `&str`
Automatic conversion | `&Vec<T>` to `&[T]` | `&String` to `&str`
Inherits methods | From `&[T]` | From `&str`

# Need to know about Strings

* Stick to `String` and `&str` for Unicode text
* When working with filename, use `std::path::PathBuf` and `&Path` instead
* When working with binary data that isn't UTF-8 encoded at all, use `Vec<u8>` and `&[u8]`
* When working with environment variables names and command-line arguments in th nativ form presented by the operating system, use `OsString` and `&OsStr`
* When interoperating with C libraries that use null-terminated strings, use `std::ffi::CString` and `&CStr` 