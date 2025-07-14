fn main() {
    let mut my_string = String::from("My name is Luis");
    println!("{}", my_string);
    my_string.push_str("inho");
    println!("{}", my_string);
    my_string.pop();
    println!("{}", my_string);

    let ma = [1, 2, 3, 4, 5];
    println!("{}", ma.len());
    println!("{:?}", ma.get(2));
}
