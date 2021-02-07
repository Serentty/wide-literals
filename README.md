# Rust wide string literals

This library provides a macro to convert string literals to UTF-16 at compile-time, allowing you to pass them into C-based APIs that expect pointers to null-terminated wide strings, such as the Windows API. This saves conversion overhead at runtime by allowing you to store UTF-16 strings directly in your executable. The syntax is as brief as I can make it, to help avoid line noise.

```rust
let my_string: &'static u16 = w!("Hello, world!");
// These strings are references to 16-bit integers, which Rust considers “close enough” to the raw pointers that APIs expect to work without casting.
MessageBoxW(null_mut(), my_string, w!("Error"), MB_ICONEXCLAMATION | MB_OK);
```

The purpose of this macro is to provide the string in exactly the form that C-based APIs need. If you want to be able to manipulate the string first, or want to use UTF-16 for non-static strings, this is not what you want, although it might still be useful for you.