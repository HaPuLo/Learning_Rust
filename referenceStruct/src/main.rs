struct Color {
    red: u8, 
    green: u8,
    blue: u8
}

fn main() {

    let blue = Color {red:0, green: 0, blue: 255};
    //Note that only using reference "&" --> "pass the reference" so that we can cha
    print_Color(&blue);
    print_Color(&blue);
}
//Define a function have argument use struct as reference
fn print_Color(c: &Color){
    println!("Color - R: {} G: {} B: {}",c.red, c.green, c.blue);
}

