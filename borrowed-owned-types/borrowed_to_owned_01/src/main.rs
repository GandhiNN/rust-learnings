/// The function creates variables of different types in Rust and demonstrates borrowing and ownership.
fn main() {
    let pet_name: String = String::from("Gary"); // Type is String
    let pet_owner_name: String = String::from("Karl"); // Type is String

    let borrowed_type_variable: &str = "this is a string";
    let owned_type_variable: String = borrowed_type_variable.to_owned();

    println!("borrowed - {}", borrowed_type_variable);
    println!("owned - {}", owned_type_variable);
}