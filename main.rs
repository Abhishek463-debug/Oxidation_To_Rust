mod begin;
mod guessinggame;

#[derive(Debug, Clone)]
struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    fn new(first_name:String, last_name: String) -> Self {
        Person{first_name, last_name}
    }
    fn full_name(&self, greeting: &str){
        println!("{}, {} {}",greeting, self.first_name, self.last_name);
    }

}

fn main() {
    println!("{}", guessinggame::play());
    println!("I love pizza!");
    begin::say_something(&mut String::from("McDonalds"));

    let mut person: Person = Person::new(String::from("Abhishek"), String::from("Swamy"));
    println!("{:?}", &person);
    person.full_name("Good morning");
    let number = "43";
    let parsed_number = number.parse::<i32>().expect("This is a parsed number");
    println!("{}", parsed_number+1);
}
