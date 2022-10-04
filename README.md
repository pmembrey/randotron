# randotron

I wrote this because I couldn't find an easy way of randomly executing Rust code. Although this is probably not something that
would need to be done on a regular basis, it's useful when you want to execute some code only once every hundred times on average
such as metric recording.

At the moment `randotron` (I couldn't think of a better name), takes a sample rate and a closure:

```rust
randotron(100, || { println!("I get printed only 1% of the time!") });
```

The sample rate is the reciprocal of the intended rate. For example, `100` would mean `1/100`, that is, once every hundred times on
average. `675` would mean that the code would be executed on average once every six hundred and seventy five times. Many libraries
represent this value as a `float` such that 1% would be `0.01` rather than `100`. The approach in `randotron` uses `u64` rather than
`f64`, to keep things simpler.

## Return type
`randotron` will evaluate to`Some(result)` (where `result` is what the closure would have evaluated to had it been executed independently)
if the closure is chosen for execution. Otherwise `randotron` will evaluate to `None`.

## Handy sample rates

You can set the sample rate to `1` which will always succeed, effectively ensuring that the closure will run every single time. Similarly
although a rate of `0` would normally cause a divide by zero error, in this case it guarantees that the closure will never execute and will
always return `None`.


