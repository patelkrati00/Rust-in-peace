pub fn even_filter(vec: &Vec<i32>) {
    for val in vec {
        if val % 2 == 0 {
            println!("Even value: {}", val);
        }
    }
}

//initialising a vector using the vec! macro
pub fn init_vector() {
    let vec = vec![1, 2, 3];
    println!("{:?}", vec);
}
