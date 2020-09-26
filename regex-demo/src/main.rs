use regex::Regex;

fn main() {
    let pattern = Regex::new(r"(?:(?P<user>[^@]+)@)*(?P<host>[^@:]+):(?P<path>\S+)").unwrap();
    let text = "howard@1.2.3.4:/home/howard";
    let text2 = "1.2.3.4:/root";
    let caps = pattern.captures(text2).unwrap();
    println!(
        "Parsing {}:\nusername: {:?}\nhost: {:?}\npath: {:?}",
        text2,
        caps.name("user"),
        caps.name("host"),
        caps.name("path")
    )
}
