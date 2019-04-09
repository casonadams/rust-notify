extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

extern crate clap;
use clap::{Arg, App};

extern crate time;

fn watch(directory: String) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch(directory, RecursiveMode::Recursive));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => println!("{:02}:{:02}:{:02} {:?}", time::now().tm_hour, time::now().tm_min, time::now().tm_sec,  event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    let matches = App::new("Simple Rust File Watcher")
      .version("0.1.0")
      .author("Cason Adams <casonadams@gmail.com>")
      .about("Watches files")
      .arg(Arg::with_name("directory")
           .short("d")
           .long("directory")
           .value_name("DIRECTORY")
           .help("Sets which directory to watch")
           .takes_value(true))
      .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let directory = matches.value_of("directory").unwrap();
    println!("Value for config: {}", directory);

    if let Err(e) = watch(directory.to_string()) {
        println!("error: {:?}", e)
    }
}
