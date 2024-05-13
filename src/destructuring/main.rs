fn main(){
    // Struct tupple destructuring
    struct Eg(String, String);
    let Eg (_a, _b) = Eg(String::from("value"), String::from("value"));

    // Struct destructuring
    struct Xa {
        name: String,
        age: u32
    }
    let p: Xa = Xa{
        name: String::from("value"),
        age: 18
    };
    let Xa {name: _name, age: _age} = &p;

}