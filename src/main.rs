use shellrecord;

fn main() {
    let res = shellrecord::run();
    if res.is_err() {
        eprintln!("{}", res.unwrap_err());
    }
}
