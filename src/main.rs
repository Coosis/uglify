use std::env;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

use uglify::populate;
use uglify::replace;

// uglify [path/file]
fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = match args.get(1) {
        None => {
            panic!("Error getting path!");
        },
        Some(s) => {
            println!("Matching at: {}", s);
            s
        }
    };
    let path = Path::new(path);
    let file_paths = get_files(path);
    for fp in &file_paths {
        if let Ok(_) = process_file(fp) {
            println!("Success! file: {}", fp.to_str().unwrap());
        }
        else {
            println!("FAILURE! file: {}", fp.to_str().unwrap());
        }
    }
}

fn process_file(file_path: &Path) -> Result<()> {
    let mut file = fs::OpenOptions::new().read(true).open(file_path)?;
    let mut contents = read_file(&mut file)?;
    let mut mp: HashMap<String, i32> = HashMap::new();
    populate(&contents, &mut mp);
    replace(&mut contents, &mp);
    println!("Read successful. Continue...");

    let mut file = fs::OpenOptions::new().write(true).truncate(true).open(file_path)?;
    write_file(&mut file, &contents)?;
    println!("Write successful!");
    Ok(())
}

fn read_file(file: &mut fs::File) -> Result<String>{
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file(file: &mut fs::File, contents: &String) -> Result<()> {
    let _ = file.write_all(contents.as_bytes());
    Ok(())
}

fn get_files(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "c" || ext == "cpp" {
                files.push(path.to_path_buf());
            } else {
                println!("Skipping file: {:?}", path);
            }
        }
    } else if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let mut sub_files: Vec<PathBuf> = get_files(&path);
                    files.append(&mut sub_files);
                } else {
                    if let Some(ext) = path.extension() {
                        if ext != "c" && ext != "cpp" {
                            println!("Skipping file: {:?}", path);
                            continue;
                        }
                        files.push(path);
                    }
                }
            }
        }
    }
    files
}
