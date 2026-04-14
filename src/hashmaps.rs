use std::collections::HashMap;

pub fn hashmp() {
    let mut users= HashMap::new();

    users.insert(String::from("xyz"), 22);
    users.insert(String::from("abc"), 20);

    let age = users.get("xyz"); //Option<>
    println!("Age of xyz is {:?}", age);

}

pub fn group_values_by_keys() -> HashMap<String,i32>{
    let input_vec = vec![(String::from("abb"),1),(String::from("bcc"),1),(String::from("cdd"),3)];
    let mut out_map = HashMap::new();

    for(key,value) in input_vec{
        out_map.insert(key, value);
    }
    return out_map;
}
