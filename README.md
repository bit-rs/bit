<div align="center">

## bit 🛰️

<i>Your satellite in the web world.  Compile once. Orbit everywhere.</i>

</div>

#### Examples
```
import io = std/io

struct Book =
  author: string,
  id: int
  
enum Option[T] =
  Some(T),
  None

enum Color =
  Rgb(int, int, int),
  Hex(string)

fn hello =
  io.print("Hello");
  io.println(", world!")
  
fn test(a: Color): int =
  match a:
    Rgb(a, b) => a + b,
    Hex(a) => len(a)
```
