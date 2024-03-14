use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation
}
// A struct is a custom data type that lets you package together and 
// name multiple related values that make up a meaningful group. Similar to an Object's data attributes.
#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

// The impl keyword is primarily used to define implementations on types. Inherent implementations are standalone,
// while trait implementations are used to implement traits for types, or other traits.
// Functions and consts can both be defined in an implementation. A function defined in an impl block can be standalone, meaning it would be called like Foo::bar(). 
// If the function takes self, &self, or &mut self as its first argument, it can also be called using method-call syntax, a familiar feature to any object oriented programmer, like foo.bar().
impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                println!("Do not serve alcohol to {}", self.name);
            }
            }
            VisitorAction::Probation => {
                println!("{} is now a probationary member", self.name);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name); 
                }
            }
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}


// Function what is your name is defined returns a string
fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}

// Functino how old are you is defined and parses line for int
fn how_old_are_you() -> i8 {
    let mut your_age = String::new();

    stdin()
        .read_line(&mut your_age)
        .expect("Failed to read line");
    let x: i8 = your_age.trim().parse().expect("Input is not an integer");
    return x
}

// Main code block
fn main() {
    // Mutable Vector defined. Simlar to arrays, but vectors are a sequential containers that store elements and are not index based. 
    // Array store a fixed-size sequential collection of elements of the same type and are index based.
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new("Steve", VisitorAction::AcceptWithNote{
        note: String::from("Lactose-free milk is in the fridge")
        }, 15),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];
    loop{
        //Loop will keep accepting user input and if the name mention is in the visitor list it will return a string with that user's welcome message
        // Else, it will return a {Name} not on list message and add the name to the list. After no input is enter. A complete list of Names is returned.
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        // Iterate through the visitor list and try to find a match
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        // If a name matches display the visitors greeting. Else return name not in list and add name to list.
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    println!("How old are you?");
                    let age = how_old_are_you();
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, age));
                }
            }
        }
    }
    // print visitor list
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}