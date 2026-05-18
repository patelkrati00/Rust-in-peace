mod vector;
mod hashmaps;
mod iterators;
mod ownership;

use vector::even_filter;


fn main() {
    let vec = vec![1, 2, 3];

     even_filter(&vec);

    // println!("{:?}", ans); //  [2]
    println!("{:?}", vec); // [1, 2, 3]

    println!("---------HashMap-----------");
    hashmaps::hashmp();
    let mp = hashmaps::group_val_by_keys();
    println!("{:?}", mp);

    println!("--------------------");
    iterators::iterate_thru_iterators();
    iterators::iterate_thru_iterator_mut();
    iterators::iterate_using_next();

    println!("--------------------");
    ownership::direct_ownership();
    ownership::ownership_in_functions();
}