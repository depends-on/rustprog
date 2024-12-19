fn main() {
    let result = rustlib::print_string("Salut salut !");
    assert_eq!(result, "Salut salut !");
    println!("Result: {}", result);
    let result2 = rustlib::print_string("salut", true);
    assert_eq!(result, "SALUT");
    println!("Result: {}", result);
}
