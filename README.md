### 🛰️ Bit
... is `WIP` programming language for building efficient software.

### Examples

```bit
fn main() {
    let name = readln("Your name?: ");
    println("Hello, " <> name);
}
```

```bit
struct House {
  street: Int,
  id: Int,
}
```

```
fn fib(n: Int) -> Int {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```
