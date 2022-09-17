fn main() {
    print_tuples();
    print_arrays();
}

fn print_tuples() {

    // Accessing same types
    println!("{}", my_tuple().0);
    println!("{}", my_tuple().1);
    println!("{}", my_tuple().2);
    println!("{}", "____________________");

    // Accessing different types
    println!("{}", my_tuple2().0);
    println!("{}", my_tuple2().1);
    println!("{}", my_tuple2().2);
    println!("{}", "____________________");

    // Destructuring
    let (var1, var2, var3) = my_tuple();
    println!("{}", var1);
    println!("{}", var2);
    println!("{}", var3);

    // format
    println!("Tuple {:?}", my_tuple());

}

fn my_tuple() -> (i32, i32, i32) {
    (1, 33, 99)
}

fn my_tuple2() -> (i32, i16, i32) {
    let my_tuple: (i32, i16, i32) = (1, 33, 99);
    my_tuple
}


fn my_array() -> [i32; 3] {
    [4; 3]
}

fn my_array2() -> [i32; 3] {
    [1, 2, 3]
}

fn print_arrays() {

    // Format 1
    let data = my_array();
    let array_size = data.len();
    for index in 0..array_size {
        println!("index : {} value: {} ", index, data[index])
    }

    // Format 2
    let data2 = my_array2();
    println!("Array: {:?}", data2);

    // Format 3
    let data3 = my_array2();
    for value in data3.iter() {
        println!("value: {}", value)
    }

}
