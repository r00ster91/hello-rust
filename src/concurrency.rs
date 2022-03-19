use std::thread;
use std::time::Duration;

pub fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hello #{} from the spawned thread!", i);

            // This sleep is when we give time for a different thread to run
            thread::sleep(Duration::from_millis(1));
            // I think it does a context switch here
        }
    });

    for i in 1..5 {
        println!("hello #{} from the main thread!", i);

        // and here too, we give time for a different thread to run
        thread::sleep(Duration::from_millis(1));
    }

    // these two threads are probably taking turns on each sleep, but that isn't guaranteed: it
    // depends on how the OS schedules the threads

    // when the main thread reached its end, the spawned thread dies too!

    // to make things more consistent, we'll use `join`
    handle.join().unwrap();
    // calling `join` on the handle blocks the thread currently running (the main thread we're in)
    // until the thread represented by the handle terminates/finishes

    // details such as where join is called can affect whether or not the threads run at the same time!
    // so if we were to place the `join` before the last `for`, we'd get non-interleaved output

    let v = vec![1, 2, 3];

    // By adding the `move` keyword before the closure,
    // we force the closure to take ownership of the values it’s using rather than allowing
    // Rust to infer that it should borrow the values
    let handle = thread::spawn(move || {
        println!("I have a vector! {:?}", v);
    });

    handle.join().unwrap();

    // mpsc stands for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    // tx and rx are traditionally used in many fields for transmitter and receiver respectively

    // with `move` we allow `tx` to be moved in this closure
    // the spawned thread needs to own the transmitting end of the channel
    // to be able to send messages through the channel.
    thread::spawn(move || {
        let message = String::from("greetings from mr. thread!");
        tx.send(message).unwrap();
    });

    // we’re using recv, which will block this (main) thread’s execution and wait
    // until a value is sent down the channel
    let received = rx.recv().unwrap();
    println!("Got something here! '{}'", received);

    // by the way,
    let mut s = "hi".to_string();
    // it seems that
    println!("{}", s);
    // is the same as
    println!("{}", &s);
    println!("{}", s);
    // !
    // still works:
    s.push_str("a");

    use std::sync::mpsc;

    let (tx1, rx1) = mpsc::channel();

    let tx2 = mpsc::Sender::clone(&tx1);
    thread::spawn(move || {
        let vals = vec![
            "more",
            "messages",
            "for",
            "ME!",
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let msgs = vec![
            "the",
            "world",
            "needs",
            "fearless",
            "concurrency"
        ];

        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for received in rx1 {
        println!("received: {:?}", received);
    }

    // now, onto the mutexes!
    // mutex is an abbreviation for mutual exclusion
    // the mutex is described as guarding the data it holds via the locking system

    use std::sync::Mutex;

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // here it's unlocked automatically!

    println!("m = {:?}", m);

    use std::{rc::Rc, sync::Arc};

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
