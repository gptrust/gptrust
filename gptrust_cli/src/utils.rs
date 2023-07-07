use std::io::Write;

pub fn dump2file(
    file_name: String,
    _directory: &Option<String>,
    body_bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(file_name.clone()).unwrap();
    file.write_all(body_bytes)
        .expect("Could not write data to file");
    Ok(())
}
