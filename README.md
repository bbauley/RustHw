# Rust_Hw1 - Write Some Rust

### 1. What did I do

I built a program that can do an operation on an arbitrary number of numbers.<br>
The operations currently available...<br>
- sum<br>
- product<br>
- gcd<br>
- lcm<br>

A few examples....
```rust
target/debug/calc sum 1 20 15 10
[1, 20, 15, 10]
sum is 46
```
```rust
target/debug/calc lcm 10 12 4
[10, 12, 4]
lcm is 60
```
### 2. How did it go

It went well for the most part. The biggest difficulty is dealing with the move feature when a variable does not have the copy trait.
Another difference that I had to get used to was that Rust checks ~~some extra things~~ *EVERYTHING* at compile time compared to C or C++.
The compiler is extremely picky but I love it. There are many ways that a programmer can accidently run into undefined behavior in C or
C++, so having the extra checks at compile time is extremely beneficial in my opinion.

### 3. How was it tested

Each function that has been written was given its own seperate test function. Each test function has a few assert_eq macros to make
sure that the function is running smoothly. <br>
The *name* of the test function has the following convention **test_name_of_function**.<br>
