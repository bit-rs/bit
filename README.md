### 🛰️ Bit
... is Low-level programming language for building efficient software.

### Examples

`!` after identifier means it is an intrisic.

```bit
fn main() {
    println!("Hello, world!");
}
```

```bit
struct House {
  street: u16,
  id: u64,
}
```

```
fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```
