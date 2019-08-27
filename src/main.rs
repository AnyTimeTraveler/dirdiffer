use std::fs::File;

fn main() {
    println!("SRC -> DST:");
    cascade("/data/media", "/mnt/media");
    println!();
    println!();
    println!();
    println!();
    println!("DST -> SRC:");
    cascade("/mnt/media", "/data/media");
}

fn cascade(src: &str, dst: &str) {
    let result = std::fs::read_dir(src);

    if result.is_err() {
        let sfile = File::open(src).expect(&format!("Error reading src file: {}", src));

        let dfile = File::open(dst);
        if dfile.is_err() {
            println!("Error reading dst file: {}", dst);
            return;
        }
        let dfile = dfile.unwrap();

        let smeta = sfile.metadata().expect(&format!("Error reading src file metadata: {}", src));
        let dmeta = dfile.metadata().expect(&format!("Error reading dst file metadata: {}", dst));

        if smeta.len() != dmeta.len() {
            println!("Mismatch len: {}\t{}\t{}", src, smeta.len(), dmeta.len());
        }
        return;
    }

    for file in result.unwrap() {
        let file = file.unwrap();
        cascade(&format!("{}/{}", src, file.file_name().to_str().unwrap()), &format!("{}/{}", dst, file.file_name().to_str().unwrap()));
    }
}
