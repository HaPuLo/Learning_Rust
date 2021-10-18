struct Color{
    red: u8,
    green: u8,
    blue: u8
}


fn main() {
    //By default Struct type was immutable, should put the  "mute" before the variable if need to
    //the value of struct
    let mut bg = Color{red: 255, green : 70, blue : 15};
    bg.red = 49;
    println!("red: {}, green: {}, blue:  {}", bg.red, bg.green, bg.blue);
}
