fn main() {
    let bytes = dysign::encrypt::tt_encrypt(&dysign::default_resources(), b"hello").unwrap();
    println!(
        "{}",
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
    );
}
