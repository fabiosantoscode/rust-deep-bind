# deep-bind

`deep-bind` helps you bind a value to any function you call, without explicitly passing it through an argument.

You might use this to hold on to configuration, a request or operation ID, or anything for which you would like to use a singleton, but are concerned about all problems that come about with global state.

## Example

Create a MyCounter context, backed by a threadlocal called MY_COUNTER.

```rust
contextual!{
    MyCounter(MY_COUNTER): u32 = 0
}

fn main() {
    println!("{}", MyCounter::clone()); /// -> 0
    MyCounter::replace_within(1, || {
        println!("{}", MyCounter::clone()); /// -> 1

        some_other_function(); // this function can also get `1`
    });
    println!("{}", MyCounter::clone()); /// -> 0
}
```

## How it works

Internally, this crate uses [`thread_local!{...}`](https://doc.rust-lang.org/std/macro.thread_local.html) to create a threadlocal with the name in parentheses, wrapped in a `RefCell`. It also creates a small utility struct with the UpperCamelCaseName you chose, to read and provide your context.
