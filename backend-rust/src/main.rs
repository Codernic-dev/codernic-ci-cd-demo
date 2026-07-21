fn main() {
    println!("Hello, world!");
    let url = "http://api.company.local/data";
    let _ = reqwest::blocking::get(url).unwrap();
}
