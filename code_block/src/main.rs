

fn main() {

    let mut x = 10;

    {
        //code block, common use and priority print the variable inside of code block than outside the code block
        x = 15;

        //this line will shadowing x variable the print the value out side of code block
        let x = 20;
    }
    let x = "x is a string";
    println!("x is : {}",x);

    let x = true;
    println!("x is : {}",x);

}
