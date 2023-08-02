use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    // define associated function on Philosopher struct
    // takes &reference to another string
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            // creates copy of String pointed to by &str
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    // methods take explicit self parameter
    fn eat(&self, table: &Table) {
        // _var used to inform compiler we know we are not using value
        // we are just locking the fork resources. Lock automatically
        // released once _left and _right are out of scope
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating", self.name);
    }
}

struct Table {
    // mutex allows control of concurrency as
    // only one thread can access contents at once
    forks: Vec<Mutex<()>>,
}

fn main() {
    // arc needed to share Table across threads
    // reference count will increase when shared
    // and decrease when thread ends
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(())
    ]});

    // Vec<T> is a growable array type
    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Friedrich Nietzsche", 3, 4),
        // left handed philosopher to prevent deadlock
        Philosopher::new("Michel Foucault", 0, 4)
    ];

    // define handles vector with type placeholder <_>
    let handles: Vec<_> =
        // into_iter creates iterator that takes ownership of each philpsopher
        // needed to pass philosopher to new threads
        philosophers.into_iter()
                    .map(|p| {
                        // increases table atomic ref count
                        let table = table.clone();

                        // move passes ownership of p to the thread
                        thread::spawn(move || {
                            p.eat(&table);
                        })
                    })
                    .collect();

    for h in handles {
        // join() waits for all threads to finish executing
        h.join()
         .unwrap();
    }
}
