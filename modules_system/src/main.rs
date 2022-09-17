use modules_system::greetings;
fn main() {
    println!("{}", greetings()); // using import
    println!("{}", modules_system::greetings()); // using absolute path
}
