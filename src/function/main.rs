#![allow(unused)]

fn sum(x: u32, y: u32) -> u32 {
    return x+y;
}

fn sum_list(list: &[i32], mut sum: i32) -> i32 {
    for &value in list.iter() {
        sum += value;
    }

    return sum;
}

fn multi_return() -> (String, String){
    return ("a".to_string(), "b".to_string());
}

fn main(){
    println!("Sum {}", sum(3, 5));
    let (a, _) = multi_return();
    println!("Multi return {}", a);

    let num_list = vec![1,2,3,4];
    let mut sum = 0;
    println!("Sum list {}", sum_list(&num_list, sum));
}