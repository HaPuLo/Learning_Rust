struct color(u8, u8, u8);

fn main() {
    
    let mut red = color (255, 0,0);

    red.2 = 50;

    println!("red is {},{},{}", red.0, red.1, red.2);
}
