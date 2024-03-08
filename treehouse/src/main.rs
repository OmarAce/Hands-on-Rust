use std::io::stdin;

#[derive(Debug)]
// A struct is a custom data type that lets you package together and 
// name multiple related values that make up a meaningful group. Similar to an Object's data attributes.
struct Visitor {
    name: String,
    greeting: String,
}

// The impl keyword is primarily used to define implementations on types. Inherent implementations are standalone,
// while trait implementations are used to implement traits for types, or other traits.
// Functions and consts can both be defined in an implementation. A function defined in an impl block can be standalone, meaning it would be called like Foo::bar(). 
// If the function takes self, &self, or &mut self as its first argument, it can also be called using method-call syntax, a familiar feature to any object oriented programmer, like foo.bar().
impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
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

// Main code block
fn main() {
    // Mutable Vector defined. Simlar to arrays, but vectors are a sequential containers that store elements and are not index based. 
    // Array store a fixed-size sequential collection of elements of the same type and are index based.
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve","Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
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
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }
    // print visitor list
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}