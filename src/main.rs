fn main() {
    let message = String::from("Say hello!!!");
    say_something(message);
}

fn say_something(data:String){
    println!("{}", data);
}
