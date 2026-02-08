# 🦎 Syntax examples

This document describes syntax of the `Geko` programming language.

### Data types

| Data type |   Rust representation   |
|-----------|-------------------------|
| int       | `i64`                   |
| decimal   | `f64`                   |
| bool      | `boo l`                 |
| string    | `String`                |
| function  | `Rc<Function>`          |
| meta type | `Rc<Type>`              |
| instance  | `Rc<Instance>`          |
| null      | `()`                    |
| native    | `Rc<Native>`            |
| module    | `Rc<Module>`            |
| any       | `Rc<dyn std::any::Any>` |

### Variable declaration
`Geko` does not support variables shadowing, so here's
a way to define variable and to reassign it.

Variable definition:
```geko
let id = value;
```

Variable assignment:
```
id = value;
```

### Binary operations
`Geko` supports following binary operations:

```geko
+ - * / % && & || | ^ > < == !=
```

### Unary operations
`Geko` supports following unary operations:

```
- !
```

### Compound operators
`Geko` supports following compound operators:

```
id += value;
id -= value;
id *= value;
id /= value;
id %= value;
id &= value;
id |= value;
```

### Value examples
Examples of the values:

| Data type | Example of the value       |
|-----------|----------------------------|
| int       | 123                        | 
| decimal   | 123.456                    |
| bool      | true / false               |
| string    | "text"                     |
| function  | fn(x, y) {} return x + y } |
| meta type | AnyDeclaredType            |
| instance  | AnyDeclaredType()          |
| null      | null                       |
| native    | declared native            |
| module    | module                     |
| any       | any_native_value           |

### Functions example
Here's an example on how you can define function in `Geko`:

```geko
fn fib(x) {
  if x <= 1 {
    return x;
  } else {
    return fib(x - 1) + fib(x - 2);
  }
}
```

Geko supports `copy environment` closures. This means that a function
can only use a copy of the `environment` as a closure. This does not mean
that reference types will be copied deeply.

```geko
fn a() {
  let x = 1;
  fn b() {
    x += 1;
  }
  b(); // x = 2, because x is in root environment
  return b;
}

let b = a(); // x got copied when function called, so `x = 1`, instead of `x = 2`
b(); // x = 2
b(); // x = 3
b(); // x = 4
```

### Classes or custom data types

Geko supports custom data types. Here is example:
```
type Dog {
  fn init() {
    self.food = 3;
    self.water = 3;
  }
  fn get_food() {
    return self.food;
  }
}
let dog = Dog();
let a = dog.get_food();
let b = dog.food;
# a == b
```

### Comments
Geko comments examples:

```
#[
Here is multiline 
comment in 
square
brackets
]#
```

```
# Here is single line comment
```

### Usings
Geko supports module using:
```
use a # import `a` as `a`
use a as b # import `a` as `b`
use a for b # import `b` from `a` directly by `shallow copying` it
use a for b, c # import multiple items
```

### Loops
Geko loops examples:

For loop with range examples.
You can use any expression instead of numbers in range.
```
for i in 0..100 {
  println(i);
}

for i in 100..0 {
  println(i);
}

for i in 0..=100 {
  println(i);
}

for i in 100..=0 {
  println(i)
}
```

While loop examples. You can see, that `Geko` supports `continue` and `break` keywords
```
let i = 0;
while true {
  if i == 100 {
    continue;
    i -= 200;
  }
  i += 1;
  if i == -199 {
    break;
  }
}
```

### Logical statements

If examples:
```
let a = scan();
if int(a) > 5 {
  ...
} else if int(a) < 5 {
  ...
} else {
  ...
}
```

### Errors handling (experimental)
```
raise "hello"
```
