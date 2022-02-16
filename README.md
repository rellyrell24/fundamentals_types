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