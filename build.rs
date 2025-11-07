use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let ui_dir = Path::new("src/ui");
    let out_dir = Path::new("target/ui");

    // Create output directory
    if !out_dir.exists() {
        fs::create_dir_all(out_dir).unwrap();
    }

    // Compile all .blp files in src/ui
    for entry in fs::read_dir(ui_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("blp") {
            let filename = path.file_stem().unwrap().to_str().unwrap();
            let output_path = out_dir.join(format!("{}.ui", filename));

            println!("cargo:rerun-if-changed={}", path.display());
            println!("Compiling Blueprint: {} → {}", path.display(), output_path.display());

            let status = Command::new("blueprint-compiler")
                .arg("compile")
                .arg("--output")
                .arg(output_path.to_str().unwrap())
                .arg(path.to_str().unwrap())
                .status()
                .expect("Failed to run blueprint-compiler");

            if !status.success() {
                panic!("Failed to compile blueprint: {}", path.display());
            }
        }
    }
}
