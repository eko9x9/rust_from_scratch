#![allow(unused)]

fn main(){
    let vec: Vec<i32> = Vec::new();
    let mut vec_mut  = vec![1, 2, 3];
    vec_mut.push(4);

    println!("1st {}", &vec_mut[0]);

    let find = &vec_mut[1];
    match vec_mut.get(8) {
        Some(find) => {
            println!("Found")
        },
        None => {
            println!("Not found")
        }
    }

    for i in &mut vec_mut {
        *i *= 2;
    }

    for i in &vec_mut {
        println!("{}", i);
    }

    println!("Length {}", vec_mut.len());
    println!("Last {:?}", vec_mut.pop());
    
}