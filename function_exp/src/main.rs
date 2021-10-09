
fn main() {
    print_number_to(20);
}

//define function with argument n 
fn print_number_to(num: u32){
    for n in 1..num{
        if check_number(n){
            println!(" {} This is even!",n);
        }
        else
        {
            println!(" {} This is odd !", n)
        }
    }
}

fn check_number(num: u32) -> bool {
    //check if num is even!   
    return num % 2 == 0;
}

