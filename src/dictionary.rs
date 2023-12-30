pub mod object{
    use std::collections::HashMap;

    pub fn new() {
        let k = String::from("name");
        let v = String::from("Adeyemi");
        let mut mapped_items: HashMap<String, String> = HashMap::new();
        mapped_items.insert(k, v);

        for (key, value) in &mapped_items {
            println!("{} -- {}", key, value);
        }
        println!("{:?}", mapped_items);
    }
}