use std::cmp::Ordering;

fn main(){
    let numb_cmp = 5.cmp(&10);

    match numb_cmp {
        Ordering::Equal => {
            println!("Numb is equal")
        },
        Ordering::Greater => {
            println!("Numb is greater")
        },
        Ordering::Less => {
            println!("Numb is less")
        }
    }
}