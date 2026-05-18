use std::collections::HashMap;

pub fn hashmp() {
    let mut users= HashMap::new();

    users.insert(String::from("xyz"), 22);
    users.insert(String::from("abc"), 20);

    let age = users.get("aaa"); //Option<>
    println!("Age of xyz is {:?}", age);

}

// pub fn group_values_by_keys() -> HashMap<String,i32>{
//     let input_vec = vec![(String::from("abb"),1),(String::from("bcc"),1),(String::from("cdd"),3)];
//     let mut out_map = HashMap::new();

//     for(key,value) in input_vec{
//         out_map.insert(key, value);
//     }
//     return out_map;
// }

struct User{
    name : String,
    age : i32
}

pub fn group_val_by_keys()-> HashMap<String,i32>{
    let user1 = User{
        name: String::from("abc"),
        age:22
    };
     let user2 = User{
        name: String::from("111"),
        age:89
    };
    let inp_vec = vec![user1,user2];
    let mut new_hm = HashMap::new();

    for(key) in inp_vec{
        new_hm.insert(key.name, key.age);
    }

    return new_hm;
}