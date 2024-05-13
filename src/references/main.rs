// references is borrowing, using syntaks &
// e.g: & var;

fn change_original(x: &mut String) {
    // changes original
    x.push_str(" and new value");
}

fn change_value(old: &mut i32, change: &i32) {
    *old = *change;
}

fn print_value(val: Vec<i32>) {
    println!("{:#?}", val);
}

fn main(){
    // no reference
    let original1 = vec![20, 20];
    print_value(original1); // values move here
    // let try_copy = original1; //cannot copy

    // reference
    let original2 = String::from("original");
    // let try_copy = original2; // trying copy, owner moved here
    let borrow = &original2; // borrowing, old owner not collapsed
    println!("{:}", borrow);

    // mutable reference
    let mut original3 = String::from("Original");
    change_original(&mut original3);
    println!("{:#?}", original3);

    let mut int = 10;
    change_value(&mut int, &200);
    println!("{:#?}", int);

}