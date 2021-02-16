use std::process::Command;
use std::{io, io::Write, path::Path};

const INTERPRETER: &'static str = "python";

/// Call external script and run computation.
/// Return the reduced dataset.
/// * `path` - The path of the dataset
pub fn compute<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    run_computation(path.as_ref())?;
    
    Ok(())
}

/// Separate function used to reduce bloat in the produced binary.
fn run_computation(path: &Path) -> Result<(), io::Error> {
    match path.to_str() {
        Some(path) => {
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg(INTERPRETER)
                    .arg(path)
                    .output()?
            } else {
                Command::new("bash")
                    .arg("-c")
                    .arg(INTERPRETER)
                    .arg(path)
                    .output()?
            };

            io::stdout().write_all(&output.stdout)?;
        }
        None => println!("Invalid path: {:?}", path),
    };

    Ok(())
}
