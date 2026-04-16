// iterating through loops

fn iterate_thru_loops() {
    let vec = vec![1, 2, 3];

    for i in vec { // under the hood, this is using into_iter()
        println!("{}", i);
    }
    // print!("{:?}",vec); //error: ownership of vec is moved to the loop, so we cannot use it after the loop
}

//type 1 - iter()
//iter() - creates an iterator that borrows the collection, so we can use the collection after the loop

pub fn iterate_thru_iterators() {
    let vec = vec![1, 2, 3];
    let iter = vec.iter();

    for i in iter {
        println!("{}", i);
    }
    println!("iterate_thru_iterators - {:?}", vec); // [1, 2, 3]  
}

//type 2 - Itermut - creates an iterator that mutably borrows the collection, so we can modify the collection after the loop

pub fn iterate_thru_iterator_mut() {
    let mut vec = vec![1, 2, 3];
    let iter = vec.iter_mut();
    for i in iter {
        *i += 1; // we can modify the collection using iter_mut
    }

    println!("iterate thru iter_mut - {:?}", vec);
}

//  iterate using .next()

pub fn iterate_using_next() {
    let vec = vec![1, 2, 3];
    let mut iter = vec.iter();

    while let Some(val) = iter.next() {
        println!("iterate_using_next {}", val)
    }
}

// type 3 - into_iter() - creates an iterator that takes ownership of the collection, so we cannot use the collection after the loop

pub fn iterate_using_into_iter() {
    let vec = vec![1, 2, 3];
    let  iter = vec.into_iter();

    for i in iter {
        print!("{}", i);
    }

    // print!("{:?}", vec); // error: ownership of vec is moved to the loop, so we cannot use it after the loop
}

