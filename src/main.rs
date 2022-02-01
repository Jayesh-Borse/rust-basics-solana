fn main() {
    println!("Hello, world!");

    //mutability
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    //if-else
    let y = 3;
    if y > 2{
        println!("Good");
    }else if y == 2{
        println!("False");
    }else {
        println!("Brrrpp")
    }

    //tuples
    let a = (1, 2.3, "Hello");
    println!("{} ,{}", a.0, a.2);

    //Array
    let x = [1, 2, 3, 4];

    //Loops
    // normal loop
    let mut f = 1;
    loop {
        f = f * 2;
        if f > 100 {
            break;
        }
        println!("{}", f);
    }

    //while loop
    while f < 1000 {
        f = f * 2;
        println!("{}", f);
    }

    //for loop
    for a in x {
        println!("{}", a);
    }

    //Match statement 
    //similar to switch case in cpp

    let q = true;
    let v = false;

    match (q, v) {
        (true, true) => println!("Great"),
        (false, false) => println!("Worst"),
        _ => println!("Invalid Choice")

    }

}
