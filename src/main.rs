fn main() {
    let result = rustlib::print_string("Salut salut !");
    assert_eq!(result, "Salut salut !");
    println!("Result: {}", result);
}
