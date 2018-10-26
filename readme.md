# matrix_2d

A canvas matrix implementation that mimics what the canvas matrix functions do.

```rust
use matrix_2d::Matrix2D;

fn main() {
  // immutable transforms
  let m1 = Matrix2D::new()
    .translate(x, y)
    .rotate(angle)
    .scale(scale_x, scale_y)
    .translate(-center_x, -center_y);

  let mut m2 = Matrix2D::new();
  m.translate_mut(x, y);
  m.rotate_mut(angle);
  m.scale_mut(scale_x, scale_y);
  m.translate_mut(-center_x, -center_y);
  println!("Matrix 1 is: {}", m1);
  println!("Matrix 2 is: {}", m2);
}
```
