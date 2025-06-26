// mkdir soroban
// cd soroban
// cargo new code_in_rust


// go to terminal -> cargo build -> it will add target, cargo.lock, and .gitignore


fn main() {

    // variables
    // let -> immutable
    // let mut -> mutable

    // basic data types -> Rust is a type language

    let x: i32 = 16;
    // to print the output out, we need to write a macro => println! macro
    println!("{}", x); // this is the first version

    let z: String = String::from("Hello, Rust!"); // mutable string
    let y: &str = "Hello, World!"; // immutable string

    println!("{}", z); // this is the second version
    println!("{}", y);

    // functions -> fn -> pub fn -> oublic fn -> private

    // pub fn event () {
    //     let name: String = String::from("Rust Event");
    //     println!("{}", name);
    // }

    let e: EventForKids = EventForKids {
        name: String::from("KidsCo"),
        date: String::from("26.06.2025"),
        number_of_participants: 1000,
        place: String::from("Istanbul")
    };

    // Print the struct values
    println!("Event: {}", e.name);
    println!("Date: {}", e.date);
    println!("Participants: {}", e.number_of_participants);
    println!("Place: {}", e.place);
}

// structs -> compiling many items in one class

struct EventForKids {
    name: String,
    date: String,
    number_of_participants: u32,
    place: String
}

// enums -> compiling errors in once class
enum ErrorsForEvent{
    NoEvent,
    CancelledEvent,
    EventType
}