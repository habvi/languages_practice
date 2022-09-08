#[allow(unused_variables, unused_assignments)]
fn main() {
    // float : default = f64
    let num_32: f32 = 4.0;
    let num_64 = 7.0;

    let mut x: f64;
    x = 10.;
    x = 10_f64;
    x = 2.5_f64;
    x = 6.02e+23;
    x = (4 as f64) + 5.5;
    x = 9f64 / 2.0;
    x = 9.0 / 2.0;

    // float can't use sort()
    // sort_by() : partial_cmp -> Some(Oredering)
    let mut v = vec![2.5, 1.26, 5.8888];
    v.sort_by(|x, y| x.partial_cmp(y).unwrap());
    assert_eq!(v, [1.26, 2.5, 5.8888]);
}