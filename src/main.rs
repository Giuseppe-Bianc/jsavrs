#[allow(unused_imports)]
use std::{env, fs, path::Path, sync::Arc};

pub mod error;
pub mod lexer;
pub mod location;
pub mod tokens;
// mod parser;*/
use error::compile_error::CompileError;

fn main() -> Result<(), CompileError> {
    /*let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];*/
    let file_path = Path::new("C:/dev/visualStudio/transpiler/Vandior/input.vn");
    let input = fs::read_to_string(file_path);
    let input = match input {
        Ok(content) => content,
        Err(e) => return Err(CompileError::IoError(e)),
    };
    let file_path_str = file_path.to_str().ok_or_else(|| {
        CompileError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid file path",
        ))
    })?;

    let (tokens, errors) = lexer::lexer_tokenize_whit_errors(input.as_str(), file_path_str);

    if !errors.is_empty() {
        for error in errors {
            eprintln!("Error: {error}");
        }
        return Err(CompileError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Lexer errors occurred",
        )));
    } else {
        for token in tokens {
            println!("{:?} at {}", token.kind, token.span);
        }
    }

    Ok(())
}
