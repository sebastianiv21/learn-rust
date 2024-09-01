# The Borrowing Concept

Borrowing is the mechanism by which Rust allows you to lend ownership of a
variable to a function or another part of your program without actually
transferring ownership of the variable. When you borrow a variable, you're
essentially saying "I want to use this variable for a little while, but I
promise I won't modify it."

Borrowing is a key concept in Rust because it allows you to write code that is
both safe and efficient. By lending ownership of a variable instead of
transferring it, Rust ensures that only one part of your program can modify the
variable at a time, which helps prevent bugs and makes it easier to reason about
your code.

We transfer ownership of a value by moving ownership from one variable to
another. Ownership can't be transferred for types that implement the Copy trait,
such as for simple values like numbers.

We can also explicitly copy values by using the cloning process. We call the
clone method and get new values that are copied, which leaves the original
values unmoved and free to still use.

Wouldn't it be nice to be able to allow functions and other variables to use
certain data without fully owning it?

This type of functionality is available by using references. References allow us
to "borrow" values without taking ownership of them.

```rust
let greeting = String::from("hello"); let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
println!("Greeting: {}", greeting); // We can still use `greeting`
```

In the previous code, greeting was borrowed by using the reference symbol (&).
The variable greeting_reference was of type string reference (&String). Since we
only borrowed greeting and didn't move ownership, greeting could still be used
after we created greeting_reference.

## References in functions

This example isn't too interesting since we're not actually using
greeting_reference for anything. Let's take a look at how we use references in
functions.

```rust
fn print_greeting(message: &String) { println!("Greeting: {}", message); }

fn main() { let greeting = String::from("Hello"); print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting`
with `&` print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again }
```

Borrowing allows us to use a value without taking full ownership. However, as
we'll see, borrowing a value means we can't do everything we can do with a fully
owned value. Mutate borrowed values

What happens if we try to mutate a value we borrowed?

```rust
fn change(message: &String) { message.push_str("!"); // We try to add a "!" to the end of our message }

fn main() { let greeting = String::from("Hello"); change(&greeting); }
```

This code won't compile. Instead we get the following compiler error: Output

error[E0596]: cannot borrow `*message` as mutable, as it is behind a `&`
reference --> src/main.rs:2:3 | 1 | fn change(message: &String) { | -------
help: consider changing this to be a mutable reference: `&mut String` 2 |
message.push_str("!"); // We try to add a "!" to the end of our message |
^^^^^^^ `message` is a `&` reference, so the data it refers to cannot be
borrowed as mutable

If you take a closer look at the compiler error, you'll see a hint about
changing our reference to be mutable by changing the type parameter from &String
to &mut String. We also need to declare our original value as mutable:

```rust
fn main() { let mut greeting = String::from("hello"); change(&mut greeting); }

fn change(text: &mut String) { text.push_str(", world"); }
```

With & borrows, known as "immutable borrows," we can read the data but we can't
change it. With &mut borrows, known as "mutable borrows," we can both read and
change the data. Borrowing and mutable references

Now we get to the real core of Rust's memory management story. Immutable and
mutable references differ in one other way that has radical effects on how we
build our Rust programs. When we borrow a value of any type T, the following
rules apply:

Your code must implement either of the following definitions, but not both at
the same time:

    One or more immutable references (&T)
    Exactly one mutable reference (&mut T)

The following code doesn't have the allowed definitions, so the compilation
fails:

```rust
fn main() { let mut value = String::from("hello");

    let ref1 = &mut value;
    let ref2 = &mut value;

    println!("{}, {}", ref1, ref2);
}
```

Output

    error[E0499]: cannot borrow `value` as mutable more than once at a time
     --> src/main.rs:5:16
      |
    4 |     let ref1 = &mut value;
      |                ---------- first mutable borrow occurs here
    5 |     let ref2 = &mut value;
      |                ^^^^^^^^^^ second mutable borrow occurs here
    6 |
    7 |     println!("{}, {}", ref1, ref2);
      |                        ---- first borrow later used here

We can even try to mix immutable references with mutable references, but the
compiler will still complain:

```rust
fn main() { let mut value = String::from("hello");

    let ref1 = &value;
    let ref2 = &mut value;

    println!("{}, {}", ref1, ref2);
}
```

Output

    error[E0502]: cannot borrow `value` as mutable because it is also borrowed as immutable
     --> src/main.rs:5:16
      |
    4 |     let ref1 = &value;
      |                ------ immutable borrow occurs here
    5 |     let ref2 = &mut value;
      |                ^^^^^^^^^^ mutable borrow occurs here
    6 |
    7 |     println!("{}, {}", ref1, ref2);
      |                        ---- immutable borrow later used here

This restriction may seem harsh at first, but this aspect of borrowing prevents
Rust code from a whole host of issues, including never having data races.

## Validate references by using lifetimes

The use of references presents a problem. The item a reference is referring to
doesn't keep track of all of its references. This behavior can lead to an issue:
when the item is dropped and its resources are freed, how can we be sure that
there are no references that point to the now freed, and invalid, memory?

Languages like C and C++ often have a problem where a pointer points to an item
that's already been freed. This problem is known as a "dangling pointer".
Fortunately, Rust eliminates this problem. It guarantees that all references
always refer to valid items. But, how does it do it?

Rust's answer to this question is lifetimes. They allow Rust to ensure memory
safety without the performance costs of garbage collection.

Consider the following snippet, which tries to use a reference whose value has
gone out of scope:

```rust
fn main() { let x; { let y = 42; x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
 } println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
}
```

The preceding code fails to compile with the following error message: Output

    error[E0597]: `y` does not live long enough
     --> src/main.rs:6:17
      |
    6 |             x = &y;
      |                 ^^ borrowed value does not live long enough
    7 |         }
      |         - `y` dropped here while still borrowed
    8 |         println!("x: {}", x);
      |                           - borrow later used here

This error occurs because a value was dropped while it was still borrowed. In
this case, y is dropped at the end of the inner scope, but x borrows it until
the println call. Because x is still valid for the outer scope (because its
scope is larger), we say that it "lives longer."

Here's the same code snippet with drawings around each variable lifetime. We
gave each lifetime a name:

    'a is the lifetime annotation for our value x.
    'b is the lifetime annotation for our value y.

```rust
fn main() {
    let x;                  // ---------+-- 'a
    {                       //          |
        let y = 42;         // -+-- 'b  |
        x = &y;             //  |       |
    }                       // -+       |
    println!("x: {}", x);   //          |
}
```

Here we can see that the inner 'b lifetime block is shorter than the outer 'a
block.

The Rust compiler can verify if the borrows are valid by using the borrow
checker. The borrow checker compares the two lifetimes at compile time. In this
scenario, x has a lifetime of 'a but it refers to a value with a lifetime of 'b.
The reference subject (y at lifetime 'b) is a shorter time than the reference (x
at lifetime 'a) so the program doesn't compile. Annotating lifetimes in
functions

As with types, lifetime durations are inferred by the Rust compiler.

There may be multiple lifetimes. When that occurs, annotate the lifetimes to
help the compiler understand which lifetime it will use to ensure the references
are valid at runtime.

For example, consider a function that takes two strings as its input parameters
and returns the longest of them:

```rust
fn main() { let magic1 = String::from("abracadabra!"); let magic2 =
String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

}

fn longest_word(x: &String, y: &String) -> &String { if x.len() > y.len() { x }
else { y } }
```

The preceding code fails to compile with an informative error message: Output

    error[E0106]: missing lifetime specifier
     --> src/main.rs:9:38
      |
    9 | fn longest_word(x: &String, y: &String) -> &String {
      |                    ----        ----        ^ expected named lifetime parameter
      |
      = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
    help: consider introducing a named lifetime parameter
      |
    9 | fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
      |                ^^^^    ^^^^^^^        ^^^^^^^        ^^^

The help text says Rust can't tell whether the reference that's being returned
refers to x or y. Neither can we. So, to help identify what the reference is,
annotate the return type with a generic parameter to represent the lifetime.

It's possible that lifetimes could be different whenever the function is called.
We don't know the concrete lifetimes of the references that will be passed to
our longest_word function, and we can't determine if the reference that will be
returned will always be a valid one.

The borrow checker can't determine if the reference will be a valid one either.
It doesn't know how the input parameters' lifetime relates to the return value's
lifetime. This ambiguity is why we need to annotate the lifetimes manually.

Luckily, the compiler gave us a hint on how to fix this error. We can add
generic lifetime parameters to our function signature. These parameters define
the relationship between the references so the borrow checker can complete its
analysis:

```rust
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String { if x.len() >
y.len() { x } else { y } }
```

You can try this code at the Rust Playground.

Make sure to declare generic lifetime parameters inside angle brackets, and add
the declaration between the parameter list and the function name.

> Note

In the signature, the return value and all the parameter references must have
the same lifetime. As such, use the same lifetime name, for example 'a. Then,
add the name to each reference in the function signature.

There's nothing special about the name 'a in this case. It would be just as fine
to use any other word, such as 'response or 'program. The important thing to
keep in mind is that all parameters and the returned value will live at least as
long as the lifetime associated with each of them.

Let's experiment with this sample code and change some values and lifetimes of
the references passed in to the longest_word function to see how it behaves. The
compiler would also reject the following snippet, but can you guess why?

```rust
fn main() { let magic1 = String::from("abracadabra!"); let result; { let magic2
= String::from("shazam!"); result = longest_word(&magic1, &magic2); }
println!("The longest magic word is {}", result); }

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String { if x.len() >
y.len() { x } else { y } }
```

You'll find this snippet at the Rust Playground.

If you guessed that this code is broken, you're right. This time, we see the
following error: Output

    error[E0597]: `magic2` does not live long enough
     --> src/main.rs:6:40
      |
    6 |         result = longest_word(&magic1, &magic2);
      |                                        ^^^^^^^ borrowed value does not live long enough
    7 |     }
      |     - `magic2` dropped here while still borrowed
    8 |     println!("The longest magic word is {}", result);
      |                                              ------ borrow later used here

This error shows that the compiler expected the lifetime of magic2 to be the
same as the lifetime of the returned value and of the x input argument. Rust
expected this behavior because we annotated the lifetimes of the function
parameters and return value by using the same lifetime name: 'a.

If we inspected the code, as humans, we would see that magic1 is longer than
magic2. We would see that the result contains a reference to magic1, which will
live long enough to be valid. However, Rust can't run that code at compile time.
It will consider both the &magic1 and &magic2 references to be possible return
values and will emit the error that we saw earlier.

The reference's lifetime that the longest_word function returns matches the
smaller of the references' lifetimes that are passed in. As such, the code
possibly includes an invalid reference and the borrow checker will disallow it.
Annotating lifetimes in types

Whenever a struct or enum holds a reference in one of its fields, we must
annotate that type definition with the lifetime of each reference that it
carries along with it.

For example, consider the following example code. We have a text string (which
owns its contents) and a Highlight tuple struct. The struct has one field that
holds a string slice. The slice is a borrowed value from another part of our
program.

```rust
#[derive(Debug)] struct Highlight<'document>(&'document str);

fn main() { let text = String::from("The quick brown fox jumps over the lazy
dog."); let fox = Highlight(&text[4..19]); let dog = Highlight(&text[35..43]);
println!("{:?}", fox); println!("{:?}", dog); }
```

The preceding code is available at the Rust Playground.

We place the name of the generic lifetime parameter inside angle brackets after
the name of the struct. This placement is so we can use the lifetime parameter
in the body of the struct definition. This instance of Highlight can't live
longer than the reference in its field because of the declaration.

In the preceding code, we annotated our struct with a lifetime called 'document.
This annotation is a reminder that the Highlight struct can't outlive the source
of the &str that it borrows, a supposed document.

The main function here creates two instances of the Highlight struct. Each
instance holds a reference to a segment of the String value owned by the
variable text:

    fox references the segment between the 4th and 19th characters of the text string.
    dog references the segment between the 35th and 43rd characters of the text string.

Also, Highlight goes out of scope before text goes out of scope, which means
that the Highlight instance is valid.

The code would print this message on the console: Output

Highlight("quick brown fox") Highlight("lazy dog")

As an experiment, try to move the value held by text out of the scope and see
what kinds of complaints the borrow checker issues:

```rust
#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(\_: String) { }

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]); let dog = Highlight(&text[35..43]);

    erase(text);

    println!("{:?}", fox);
    println!("{:?}", dog);
}
```

You can find this failing code snippet at the Rust Playground.
