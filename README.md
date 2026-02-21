### 🛰️ Bit
... is Low-level programming language for building efficient software.

### Examples

`!` after identifier means it is an intrinsic.

```bit
fn main() {
    let name = readln!("Your name?: ");
    println!("Hello, " <> name);
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
