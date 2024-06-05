mod io;
mod error;

fn main() {
    let req = io::next_request().unwrap();
    eprintln!("Received request: \n{}", req);
}
