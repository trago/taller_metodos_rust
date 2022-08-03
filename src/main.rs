fn calc_area(width: i32, height: i32) -> i32 {
    width * height
}

fn main() {
    let width = 30;
    let height = 40;

    let area = calc_area(width, height);

    println!("the area of rectagle is {area}");
}
