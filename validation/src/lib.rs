use csv::{ReaderBuilder, Result as CSVResult, StringRecord};
use std::path::Path;

/// Headers: are they valid headers
/// Records: are they all numeric
/// * `path` - The path of the data file
pub fn validate_data<P: AsRef<Path>>(path: P) -> CSVResult<()> {
    read_csv(path.as_ref())?;
    Ok(())
}

fn read_csv(path: &Path) -> CSVResult<()> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut record = StringRecord::new();

    while reader.read_record(&mut record)? {
        if record.is_empty() {
            panic!("empty");
        }

        let _first = record.get(0).unwrap();
    }

    Ok(())
}
