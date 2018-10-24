# matrix_2d

A canvas matrix implementation that mimics what the canvas matrix functions do.

```rust
use matrix_2d::Matrix2D;

fn main() {
  let m = Matrix2D::new()
    .translate(x, y)
    .rotate(angle)
    .scale(scale_x, scale_y)
    .translate(-center_x, -center_y);

  println!("Matrix is: {}", m);
}
```
