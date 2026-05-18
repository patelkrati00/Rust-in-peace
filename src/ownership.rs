pub fn direct_ownership(){
    let s1 = String::from("hello");
    let s2 = s1; // ownership of s1 is moved to s2, so we cannot use s1 after this line
    println!("{}", s2); // hello
    // println!("{}", s1); // error: ownership of s1 is moved to s2
}

pub fn ownership_in_functions(){
    let s1 = String::from("hello");
    takes_ownership(s1); // ownership of s1 is moved to the function, so we cannot use s1 after this line
    // println!("{}", s1); // error: ownership of s1 is moved to the function

    let x = 5;
    makes_copy(x); // ownership of x is copied to the function, so we can use x after this line
    println!("{}", x); // 5
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}