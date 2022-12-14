use regex::Regex;

struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn sayhi(&self) -> String {
        format!("Meunome é {} e a minha idade é {}", self.name, self.age)
    }
}

fn struct_say_hi() -> String {
    let person = Person { name: String::from("John"), age: 31 };
    format!("Struct say Hi {}", format!("{}", person.sayhi()))
}

fn destructuring() -> String {
    let (variable1, variable2) = (8, "teste");

    const MY_CONST: &str = "teste23";

    format!("var 1 {} var 2 {} const {}", variable1, variable2, MY_CONST)
}

fn regex_match(input_string: &str) -> String {
    let my_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    return format!("Did our date match? {}", my_regex.is_match(input_string));
}


fn mutable_variable() -> String {
    let mut _my_var = "teste";
    _my_var = "test2";

    format!("mutable variable {}", _my_var)
}


fn ownership() {

    let string1 = "string1".to_string();
    let string2 = &string1;
    println!("{}", string2);
    consume(&string1);
    println!("{}", string1);
}


fn consume(_var1: &String){
}

fn main() {
    println!("{}", regex_match("2014-01-01"));
    println!("{}", destructuring());
    println!("{}", struct_say_hi());
    println!("{}", mutable_variable());

    ownership();
}
