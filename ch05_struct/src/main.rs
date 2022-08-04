fn main() {
    let weight1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels. ",
        area(weight1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
