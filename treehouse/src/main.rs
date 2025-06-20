#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}
#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the house, {}", self.name);
                println!("Note: {}", note);
                if self.age < 21 {
                    println!("No alcohol for {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("{} is not allowed", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read input line for your name");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("kurt", VisitorAction::Accept, 52),
        Visitor::new("fred", VisitorAction::Refuse, 52),
        Visitor::new(
            "anna",
            VisitorAction::AcceptWithNote {
                note: String::from("Looking good in the PJs"),
            },
            53,
        ),
        Visitor::new(
            "sophia",
            VisitorAction::AcceptWithNote {
                note: String::from("Smash the patriarchy"),
            },
            20,
        ),
    ];

    println!("Hello, what's your name?");

    let name = what_is_your_name();

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => {
            println!("Unrecognized name.");
            visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
    }

    println!("The final vector of visitors:");
    println!("{:#?}", visitor_list);
}
