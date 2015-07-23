//Derek Prince
//Following an eample for concurrency via The Dining Philosophers (Dijkstra)
//from: https://doc.rust-lang.org/stable/book/dining-philosophers.html
//
//On a side note, this scenario is like someone thinking of the punchline 
//before coming up with the joke. Except that the punch line is concurrency,
//and we already have too much concurrency in punch lines.

use std::thread;                //multi threading
use std::sync::{Mutex, Arc};    //sync for Mutex and Arc only

struct Philosopher {
    name: String,
    left: usize,    //usize, conveniently, is the type used to index vectors.
    right: usize,
}

//impl allows defining things on philosopher structs.
//In this case, a constructor, called new, that takes an
//external string reference and sets the structs internal `name`
//string to the external name &str by use of .to_string()
//This is immediately used in fn main() to create the 5 philosophers w/ name arguments
impl Philosopher {  
    fn new(name: &str, left: usize, right: usize) -> Philosopher { //returns a Philosopher struct
        Philosopher { //last expression, austomatically returned
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    
    //the explicit statement of &self in eat is what makes it a method of
    //Philosopher, while new is only an associated function, called by ::
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap(); 
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        //can I just say how much nicer it is to use 'self' over 'this'
        //'this' is a piece of garbage (and it's not even collected.)
        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

//
struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    //stuffs the philosophers into a vector rather than 5 objects.
    //the vector is called philosopher
    //(duh)
    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 0, 1),
        Philosopher::new("Gilles Deluze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Friedrich Nietzsche", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
        //alternate form without ::new() would look like...
        //let p6 = Philosopher { name: "Asswrangler Adolf".to_string() };
    ];
   
    //hold me close, sanity
    //'_' is a type placeholder in the Vector declaration for handles
    //into_iter() creates an iterator on philosopher (the vector)
    //to take ownership of each object philosopher
    //with this newly minted iterator, a map is called on it which takes
    //a 'closure as an argument and calls that closure on each element in turn.'
    //Alrighty.
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || { 
            p.eat(&table);
        }) //creates a thread with a closure as an argument, and executes it with impunity
           //move indicates that the cosure is taking ownership of the values it's capturing
           //(p variable in the map function, for the most part)
    }).collect();
    //collect grabs all map call results and stores them in the vector we so 
    //cleverly made - handles. Inferred type at it's finest here
    //stores a bunch of handles from the hread::spawn calls to access each thread.

    //going through the colection in handles,
    //join each thread. .join() ensures that each thread finishes execution before 
    //the combining executes.
    //and voila! multithreading.
    for h in handles {
        h.join().unwrap();
    }
}
