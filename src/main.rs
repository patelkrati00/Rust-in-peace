mod vector;
mod hashmaps;
mod iterators;

use vector::even_filter;


fn main() {
    let vec = vec![1, 2, 3];

    let ans = even_filter(&vec);

    println!("{:?}", ans); //  [2]
    println!("{:?}", vec); // [1, 2, 3]

    //--------------------
    hashmaps::hashmp();
    let mp = hashmaps::group_values_by_keys();
    println!("{:?}", mp);

    //--------------------
    iterators::iterate_thru_iterators();
    iterators::iterate_thru_iterator_mut();
    iterators::iterate_using_next();
}