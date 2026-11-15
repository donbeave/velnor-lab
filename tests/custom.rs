fn main() {
    if std::env::args().any(|a| a == "--list") {
        return;
    }
    eprintln!("custom harness ran and failed");
    std::process::exit(1);
}
