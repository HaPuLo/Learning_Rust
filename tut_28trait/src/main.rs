

struct Person{
    name: String,
    age : u8
}

trait HasVoiceBox{
    //speak
    fn speak (&self);
    fn can_speak (&self) -> bool;
    
}

//implement trait for person
//define the function speak and Can_speak for person
impl HasVoiceBox for Person {
    fn speak (&self){
        println!("Hello, My name is {}", self.name);
    }
    fn can_speak (&self) -> bool {
        if self.age > 0{
            return true;
        } return false; 
    }
}

fn main (){
    let person = Person{
        name: String::from("Bob"),
        age: 34
    };

    println! ("Can {} speak {} ", person.name, person.can_speak());
}
