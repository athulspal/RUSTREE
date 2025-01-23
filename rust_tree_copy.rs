use std::fs;
use std::path::{Path, PathBuf};
use std::io;

fn create_rust_tree(source_path: &Path, dest_path: &Path) -> io::Result<()> {
    if !source_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source path is not a directory",
        ));
    }

    fs::create_dir_all(dest_path)?;

    process_directory(source_path, dest_path)?;

    Ok(())
}

fn process_directory(source_dir: &Path, dest_dir: &Path) -> io::Result<()> {
    for entry_result in fs::read_dir(source_dir)? {
        let entry = entry_result?;
        let entry_path = entry.path();
        let entry_name = entry.file_name();

        let dest_entry_path = dest_dir.join(&entry_name);

        if entry_path.is_dir() {
            fs::create_dir_all(&dest_entry_path)?;
            process_directory(&entry_path, &dest_entry_path)?;
        } else {
            process_file(&entry_path, &dest_entry_path)?;
        }
    }
    Ok(())
}

fn process_file(source_file: &Path, dest_file: &PathBuf) -> io::Result<()> {
    let file_name = source_file.file_name().unwrap();
    let file_name_str = file_name.to_string_lossy();

    let mut dest_file_name = PathBuf::from(file_name_str.as_ref());

    if let Some(extension) = source_file.extension() {
        if extension == "ts" || extension == "js" {
            dest_file_name.set_extension("rs");
        }
    }

    let final_dest_file_path = dest_file.with_file_name(dest_file_name);
    fs::File::create(final_dest_file_path)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rust_tree_copy <source_directory>");
        return Ok(());
    }

    let source_dir_str = &args[1];
    let source_path = Path::new(source_dir_str);

    if !source_path.exists() {
        eprintln!("Error: Source directory '{}' does not exist.", source_dir_str);
        return Ok(());
    }

    let dest_dir_str = format!("{}_rs", source_dir_str);
    let dest_path = Path::new(&dest_dir_str);

    println!("Creating rust tree copy from '{}' to '{}'", source_dir_str, dest_dir_str);

    match create_rust_tree(source_path, dest_path) {
        Ok(_) => println!("Rust tree structure created successfully in '{}'", dest_dir_str),
        Err(e) => eprintln!("Error creating rust tree structure: {}", e),
    }

    Ok(())
}
