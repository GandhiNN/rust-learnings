fn main() {
    let pet_name: String = String::from("Gary"); // Type is String
    let pet_owner_name: String = String::from("Karl"); // Type is String

    // Assigning the same value between variables pet_name and pet_alias
    let pet_alias = pet_name;
    let pet_owner_alias = pet_owner_name;

    // This won't compile
    println!("Pet Name: {}", pet_name);
    println!("Pet Alias: {}", pet_alias);

    println!("Pet Owner Name: {}", pet_owner_name);
    println!("Pet Owner Alias: {}", pet_owner_alias);
}