# rust-deep-bind

rust-deep-bind helps you bind a value to any function you call, without explicitly passing it through an argument.

You might use this to hold on to configuration, a request or operation ID, or anything for which you would like to use a singleton, but are concerned about all problems that come about with global state.

Example:

```rust
contextual!{
    MyCounter(MY_COUNTER): u32 = 0
}

println!("{}", MyCounter::clone()); /// 0
MyCounter::replace_within(1, || {
    println!("{}", MyCounter::clone()); /// 1
});
println!("{}", MyCounter::clone()); /// 0
```
