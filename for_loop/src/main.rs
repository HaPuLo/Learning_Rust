fn main() {

    let numbers_range = 1..5;
    //define a vector
    let animals = vec!["cat", "dog", "ant"];
    
    for n in numbers_range{

        println!("n number is : {}",n);
    }
   for (index, a) in animals.iter().enumerate(){
       println! ("the animal of Index {} is {}",index, a);
   }
}
