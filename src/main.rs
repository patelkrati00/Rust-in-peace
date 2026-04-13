mod vector;

use vector::even_filter;

fn main() {
    let vec = vec![1, 2, 3];

    let ans = even_filter(&vec);

    println!("{:?}", ans); // [2]
    println!("{:?}", vec); // [1, 2, 3]
}