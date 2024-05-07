Attempting to import C++ Win32 Compiled .dll via Rust FFI. Written with the help of ChatGPT.

Compile native/main.cpp from project root:
    g++ -shared -fPIC -o native/main.dll native/main.cpp (then cargo build / cargo run to test / run after)