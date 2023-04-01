mod cc;
use std::{thread};

use std::sync::mpsc;
use core::time::Duration;
use crate::cc::terminal::Terminal;
use crate::cc::pool::Pool;


fn main() {
    let (tx, rx) = mpsc::channel();
    let terminal = Terminal::new();

     thread::spawn(move || {
        let pool = Pool::new();
        let update = cc::format(&pool);
        tx.send(update).unwrap();
        thread::sleep(Duration::from_secs(1));
        let update = String::from("Test1 : 3\n Test2: 3");
        tx.send(update).unwrap();
        thread::sleep(Duration::from_secs(1));
        let update = String::from("Test1 : 5\n Test2: 4");
        tx.send(update).unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    for received in rx {
       terminal.update(received); 
    }

    println!("Done!");
}
