// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)


use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    // We use an Arc as we can't pass the mutex to the threads'
    // closures without them taking ownership
    //
    // clone of an (atomic) reference (count) to the mutex
    // let status_shared = status.clone();
    // Can use above, but convention is below (reminding us we are not deep copying)
    let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut s = status_shared.lock().unwrap();
            s.jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }

    status.lock().unwrap().jobs_completed = 0;
    for i in 0..10 {
        // each thread's closure takes possession of a clone of the status Arc
        let status_shared = Arc::clone(&status);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis((10-i)*100));
            println!("greetings from thread {}",i);
            let mut s = status_shared.lock().unwrap();
            s.jobs_completed += 1;
        });
    };
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(100));
    }
}
