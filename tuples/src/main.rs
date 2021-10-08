


fn main() {
    //tuple define by parenthesis ()
    let tup1 = (1, 2.2, true, "computer",(5,6,7));

    //assigned array for tuple
    let (a, b, c, d, e) = tup1;

    //access the element
    println!("The second element of tuple: {} ", (tup1.4).2);

    //print the array assigned the tuple
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
    println!("d is {}", d);
    println!("e is {}", e.2);
}
