use std::fs;
fn main() {
    if let Err(_) = fs::read_dir("logs") {
        fs::create_dir("logs")
            .expect("Unable to create directory 'logs'");
    }

    if let Err(_) = fs::read_dir("references") {
        fs::create_dir("references")
            .expect("Unable to create directory 'references'");
    }
}
