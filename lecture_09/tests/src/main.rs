fn doubleme(inp: &Vec<f64>) -> Vec<f64> {
    let mut nv = inp.clone();
    for (i, x) in inp.iter().enumerate() {
         nv[i] = *x * 2.0 + 0.01;   // use this line to show failed tests
//         nv[i] = *x * 2.0;
    }
    nv
}

#[test]
fn test_doubleme_positive() {
    let v = vec![1.0, 2.0, 3.0];
    let w = doubleme(&v);
    for (x, y) in v.iter().zip(w.iter()) {
        assert_eq!(*y, 2.0 * *x, "Element is not double");
    }
}
#[test]
fn test_doubleme_negative() {
    let v = vec![-1.0, -2.0, -3.0];
    let w = doubleme(&v);
    for (x, y) in v.iter().zip(w.iter()) {
        assert_eq!(*y, 2.0 * *x, "Negative element is not double");
    }
}
#[test]
fn test_doubleme_zero() {
    let v = vec![0.0];
    let w = doubleme(&v);
    for (x, y) in v.iter().zip(w.iter()) {
        assert_eq!(*y, 2.0 * *x, "Zero element is not double");
    }
}
#[test]
fn test_doubleme_empty() {
    let v: Vec<f64> = vec![];
    let w = doubleme(&v);
    assert_eq!(w.len(), 0, "Empty Vector is not empty");
}

fn main() {
    let v: Vec<f64> = vec![2.0, 3.0, 4.0];
    let w = doubleme(&v);
    println!("V = {:?} W = {:?}", v, w);

    println!("{}", v[0]);
    //println!("{}", v[100]);
}
