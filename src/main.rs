fn main() {
    if let Err(e) = header::get_args().and_then(header::run) {
        eprintln!("header: {}", e);
        std::process::exit(1);
    }
}
