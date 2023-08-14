fn print_str(str: String) {
    println!("Print, {}", str);
}

fn print_return_str(str: String) -> String {
    println!("Print and return, {}", str);
    return str.to_string();    
}

fn change_string(str: &mut String) {
    str.push_str(" has new string");
    println!("{}", str);
}

fn main() {
    let mut str = String::from("string");
    // clone str and owner not collapsed
    let clone_str = str.clone();
    // create new owner from fn return string
    let return_str = print_return_str(str.to_string());

    print_str(clone_str);
    print_str(return_str);
    // value changed
    change_string(&mut str);
    print_str(str);
}