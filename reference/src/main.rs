fn main() {

    let mut x = 10;

    //xr just reference the value of x
    //let xr = &x;
    //println!("x is : {} ",xr);

    //Change the value with reference
    let dom = &mut x;
    *dom += 1;

    //Cannot change the value of references, unless it reference the mutable of reference "& mut x"
    println!("x is : {} ",x);

}
