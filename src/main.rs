use std::fmt;

fn main() {

    //Si hago rusct a main.rs se hace un ejecutable
    //y haciendo ./main.rs lo ejecuto en terminal!
    println!("Hello, world!");

    let number: f64 = 1.0;
    let width: usize = 5;

    #[allow(dead_code)]
    struct Structure(i32);
    fmt::Debug.
    println("This struct `{}` won't print...", Structure(3));
    /*
    Activities
    Fix the issue in the above code (see FIXME) so that it runs without error.
        println!("My name is {0},{1},{0}","Bond","James");
    Try uncommenting the line that attempts to format the Structure struct (see TODO)

    Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown.
    For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi.
    (Hint: you may need to check the std::fmt documentation for setting the number of
    decimals to display)
     */

}
