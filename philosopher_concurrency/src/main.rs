//Derek Prince
//Following an eample for concurrency via The Dining Philosophers (Dijkstra)
//from: https://doc.rust-lang.org/stable/book/dining-philosophers.html
//
//On a side note, this scenario is like someone thinking of the punchline 
//before coming up with the joke. Except that the punch line is concurrency,
//and we already have too much concurrency in punch lines.

struct Philosopher {
    name: String,
}

//impl allows defining things on philosopher structs.
//In this case, a constructor, called new, that takes an
//external string reference and sets the structs internal `name`
//string to the external name &str by use of .to_string()
//This is immediately used in fn main() to create the 5 philosophers w/ name arguments
impl Philosopher {  
    fn new(name: &str) -> Philosopher { //returns a Philosopher struct
        Philosopher { //last expression, austomatically returned
            name: name.to_string()
        }
    }
    
    //the explicit statement of &self in eat is what makes it a method of
    //Philosopher, while new is only an associated function, called by ::
    fn eat(&self) {
        println!("{} is done eating.", self.name);
        //can I just say how much nicer it is to use 'self' over 'this'
        //'this' is a piece of garbage (and it's not even collected.)
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Baruch Spinoza"),
        Philosopher::new("Gilles Deluze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Friedrich Nietzsche"),
        Philosopher::new("Michel Foucault"),
        //alternate form without ::new() would look like...
        //let p6 = Philosopher { name: "Asswrangler Adolf".to_string() };
    ];

    for p in &philosophers {
        p.eat();
    }
}
