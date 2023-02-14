use clap::Parser;
use std::sync::Arc;
use std::thread;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("world"))]
    name: String,

    /// Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: u8,
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.name);

    let shared_name = Arc::new(args.name);

    let mut handles = vec![];

    for i in 0..args.threads {
        let name = shared_name.clone();
        let handle = thread::spawn(move || {
            println!("Thread {} says: Hello {}", i, name);
        });
        handles.push(handle);
    }

    for th in handles {
        th.join().unwrap();
    }
}
