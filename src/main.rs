fn main() {
    let angle: f64 = 2.0;
    let side_length = 80.0;

    let hypotenuse = side_length / angle.sin();

    println!("Hypotenuse: {}", hypotenuse);
}

// fn main() {
//     let x: f64 = 6.0;
//
//     let a = x.tan();
//     let b = x.sin() / x.cos();
//
//     assert_eq!(a, b);
// }
