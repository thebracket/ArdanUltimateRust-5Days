use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("mytile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

fn main() {
    match maybe_read_a_file() {
        Ok(text) => println!("File contents: {text}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
    match file_to_uppercase() {
        Ok(text) => println!("File contents: {text}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
    let _ = file_to_uppercase();
}
