# Specish

I don't want to obsess over a really long spec, but I do need to write some things down.

Why is designed (right now) to be a scripty-style language but with native compilation. Thus, it has scripty features.

## Structure

A Why program consists of a sequence of statements and declarations. The final statement may be an expression that returns a value (this is like Rust's own function semantics).

### "Hello, World" Example

```
"Hello, World"
```

### A slightly more useful example

```
let name = stdin.lines().next().unwrap();
"Hello, {name}"
```
