### 1\. What is shadowing in Rust, give an example, and also explain shadowing vs mutability.

#### **Shadowing:**

Shadowing in Rust allows you to declare a new variable with the same name as a previous variable. The new variable overshadows the previous one, allowing you to perform different operations or change the type.

##### Example:

```rust
fn main() {
    let x = 5;
    let x = x + 1; // x is now 6
    let x = "Hello"; // x is now a string

    println!("{}", x); // prints "Hello"
}
```

-   **Explanation**: In this example, `x` is first assigned the value `5`. It is then shadowed by a new `x` with the value `6`, and then shadowed again by a new `x` with the value `"Hello"`. Each new `x` declaration can have a different type.

#### **Mutability:**

Mutability in Rust allows you to change the value of a variable after it is initialized, but the type of the variable cannot change.

##### Example:

```rust
fn main() {
    let mut y = 5;
    y += 1; // y is now 6

    println!("{}", y); // prints 6
}
```

-   **Explanation**: In this example, `y` is declared as mutable with the keyword `mut`. This allows us to change its value from `5` to `6`.

#### **Difference:**

-   **Shadowing** allows redeclaring a variable and changing its type.
-   **Mutability** allows changing the value of a variable, but not its type.

### 2\. What is ownership in Rust? Explain the Borrowing Rules.

#### **Ownership:**

Ownership is a central concept in Rust that ensures memory safety without a garbage collector. Each value in Rust has a variable that's called its owner. There can only be one owner at a time, and when the owner goes out of scope, the value is dropped.

##### Example:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // error: value borrowed here after move
    println!("{}", s2); // prints "hello"
}
```

-   **Explanation**: In this example, `s1` owns the string `"hello"`. When `s1` is assigned to `s2`, `s1` is moved to `s2`, and `s1` is no longer valid.

#### **Borrowing:**

Borrowing allows you to use a reference to a value without taking ownership. There are two types of borrowing: immutable and mutable.

##### Immutable Borrowing:

You can have multiple immutable references to a value, but you cannot have a mutable reference while immutable references exist.

##### Example:

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s; // immutable borrow
    let r2 = &s; // another immutable borrow

    println!("{} and {}", r1, r2); // prints "hello and hello"
}
```

-   **Explanation**: In this example, `r1` and `r2` are immutable references to `s`. Multiple immutable references are allowed.

##### Mutable Borrowing:

You can have only one mutable reference to a value at a time, and no immutable references can coexist with a mutable reference.

##### Example:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s; // mutable borrow

    // println!("{}", s); // error: cannot borrow `s` as immutable because it is also borrowed as mutable
    println!("{}", r1); // prints "hello"
}
```

-   **Explanation**: In this example, `r1` is a mutable reference to `s`. Only one mutable reference is allowed, and no immutable references can coexist.

### Rules of borrowing
- There can me many immutable references at the same time
```rust
fn main() {
    let  s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s1;

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}
// No errors
```
- There can be only one mutable reference  at a time
```rust
fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    let s3 = update_word(&mut s1);

    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
// Error
```
- If there is a mutable reference , you can’t have another immutable reference either.
```rust
fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    let s3 = &s1;

    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
```
- But you can have multiple mutable and mutable with immutable borrowings only if the scope of one ends before occurance of the second.
```rust
fn main(){
      let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    s2.push_str(" World");
    println!("{}", s2);
    let s3 = &s1;
    println!("{}", s3);

}
```

### 3\. How does Rust handle Error Propagation?

Rust handles error propagation using the `Result` enum, which can represent either a success (`Ok`) or an error (`Err`). The `?` operator simplifies error propagation by returning the error if it occurs.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

##### Example:

```rust
use std::fs::File;

fn main() {
  let file_result = fs::read_to_string("hello.txt");
    match file_result{
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

-   **Explanation**: The `read_file` function attempts to open a file and read its content. If any operation fails the `main` function handles the `Result` by matching on `Ok` and `Err`.

### Unwraps
- Incase you are ok with runtime errors (crashing the process while it runs if an error happens), then you can unwrap a Result.

```rust
use std::fs;

fn main() {
    let file_result = fs::read_to_string("hello.txt");
    print!("{}", file_result.unwrap());
}
```
