use std::fs::{File, read_dir};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} source destination", args[0]);
        return;
    }
    let src = &args[1];
    let dst = &args[2];
    println!("SRC -> DST:");
    cascade(src, dst, true);
    println!();
    println!();
    println!();
    println!();
    println!("DST -> SRC:");
    // no need to compare sizes on the second pass, as we're only interested in the files that exist in the destination and not in the source
    cascade(dst, src, false);
}

fn cascade(src: &str, dst: &str, compare_size: bool) {
    let result = read_dir(src);

    let dfile = File::open(dst);

    if dfile.is_err() {
        println!("File missing or unreadable: {}", dst);
        return;
    }
    let dfile = dfile.unwrap();

    match result {
        Ok(folder) => {
            for file in folder {
                let file = file.expect(&format!("Error unwrapping DirEntry in subdirectory of: {}", src));
                cascade(&format!("{}/{}", src, file.file_name().to_str().unwrap()), &format!("{}/{}", dst, file.file_name().to_str().unwrap()), compare_size);
            }
        }
        Err(_) => { // meaning it's a file and not a directory
            if compare_size {
                let sfile = File::open(src).expect(&format!("Error reading src file: {}", src));

                let smeta = sfile.metadata().expect(&format!("Error reading src file metadata: {}", src));
                let dmeta = dfile.metadata().expect(&format!("Error reading dst file metadata: {}", dst));

                if smeta.len() != dmeta.len() {
                    println!("Mismatch len: {}\t{}\t{}", src, smeta.len(), dmeta.len());
                }
            }
        }
    }
}
