use std::ops::Add;

fn sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
fn main(){
    println!("Sum generic {}", sum_gen(3, 5));
}