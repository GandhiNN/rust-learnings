fn coerce_success(data: &str) {
    println!("{}", data);
}

fn coerce_fail(data: &String) {
    println!("{}", data);
}

fn main() {
    let s = "hello_world";
    let mut mutable_string = String::from("hello");

    coerce_success(&mutable_string); // we put a &String, it compiles succesfully
    coerce_fail(s); // we put a &str, it does not compile

}