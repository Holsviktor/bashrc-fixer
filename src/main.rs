use inotify::{
    Inotify,
    WatchMask
};
use std::{
    path::Path,
    fs::File,
    io::Write,
};

fn main() {
    let home = std::env::var("HOME").expect("Failed to find home directory.");
    let bashrc = Path::new(&home).join(Path::new(".bashrc"));

    let mut inotify = Inotify::init().expect("Failed to init inotify.");


    loop {
        inotify.watches().add(bashrc.clone(), WatchMask::MODIFY).expect("Failed to watch bashrc");
        let mut buffer = vec![0 ; 1024];
        let _events = inotify.read_events_blocking(&mut buffer).expect("Error whilst reading events.");
        
        let mut bashrc_file = File::options().append(true).open(bashrc.clone()).expect("Failed to open file.");

        bashrc_file.write("\nsl\n".as_bytes()).expect("Failed to write to bashrc");
    }
}
