# Neotron API

This crate defines the API between the Neotron OS and any Neotron Applications running on it.

If you are writing a Neotron Application, you might prefer to use the Neotron SDK, which wraps up this API into something slightly more useable.

Note that this API must be FFI-safe, because the OS and the Application may be compiled with different versions of Rust.


