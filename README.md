# Rust wide string literals

This library provides a macro to convert string literals to UTF-16 at compile-time, allowing you to pass them into C-based APIs that expect pointers to null-terminated wide strings, such as the Windows API. This saves conversion overhead at runtime by allowing you to store UTF-16 strings directly in your executable. The syntax is as brief as I can make it, to help avoid line noise.

```rust
let my_string: *const u16 = w!("Hello, world!");
// Strings are already in the type that APIs expect.
MessageBoxW(null_mut(), my_string, w!("Error"), MB_ICONEXCLAMATION | MB_OK);
```