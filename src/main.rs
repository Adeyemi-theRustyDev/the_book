mod err_propagation;

use err_propagation::{read_uname, read_username};
fn main() {
    let file_cnts = read_uname("./src/person.rs").unwrap();
    let other_file_contents = read_username("./src/dictionary.rs").unwrap();

    println!("**************************FILE CONTENT******************************");
    println!("{}", file_cnts);
    println!("**************************FILE CONTENT******************************");

}

