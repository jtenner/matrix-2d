use crate::matrix_2d::Matrix2D;

fn setup() -> Matrix2D {
  Matrix2D::new()
}

#[test]
fn new_returns_identity() {
  let m = setup();
  assert_eq!(m.a, 1.0);
  assert_eq!(m.b, 0.0);
  assert_eq!(m.c, 0.0);
  assert_eq!(m.d, 1.0);
  assert_eq!(m.e, 0.0);
  assert_eq!(m.f, 0.0);
}

#[test]
fn translate_mut_performs_translation() {
  let mut m = setup();
  assert_eq!(m.e, 0.0);
  assert_eq!(m.f, 0.0);
  m.translate_mut(100.0, 100.0);
  assert_eq!(m.e, 100.0);
  assert_eq!(m.f, 100.0);
}

#[test]
fn translate_performs_translation_on_new_matrix() {
  let m = setup();
  let n = m.translate(100.0, 100.0);
  assert_ne!(m.e, n.e);
  assert_ne!(m.f, n.f);
}

#[test]
fn scale_mut_performs_scale() {
  let mut m = setup();
  m.scale_mut(2.0, 3.0);
  assert_eq!(m.a, 2.0);
  assert_eq!(m.b, 0.0);
  assert_eq!(m.c, 0.0);
  assert_eq!(m.d, 3.0);
}

#[test]
fn scale_performs_scale_on_new_matrix() {
  let m = setup();
  let n = m.scale(2.0, 3.0);
  assert_ne!(m.a, n.a);
  assert_ne!(m.d, n.d);
  assert_eq!(n.a, 2.0);
  assert_eq!(n.d, 3.0);
}
