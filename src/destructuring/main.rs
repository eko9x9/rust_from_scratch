fn main(){
    // Struct tupple destructuring
    struct Eg(String, String);
    let Eg (a, b) = Eg(String::from("value"), String::from("value"));

    // Struct destructuring
    struct Xa {
        name: String,
        age: u32
    }
    let p: Xa = Xa{
        name: String::from("value"),
        age: 18
    };
    let Xa {name, age} = &p;

}