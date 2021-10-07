//Create conflic code hihi
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

fn main() {

    //define variable using enum
    let player_direction:Direction = Direction::DOWN;
 
    match player_direction{
        Direction::UP => println!("We are going up!"),
        Direction::DOWN => println!("we are going down!"),
        Direction::LEFT => println!("Turn left"),
        Direction::RIGHT => println!("TO the right !! go go!")
    }
}
