//! Processes the failing doctest code in `../doc-example-parts/` into functions that can be
//! compiled into the `panic-example-test` binary.
//! This binary is then used to verify that the panic output displayed in the documentation matches
//! the actual output of running the program.

use std::fs;
use std::io::Write as _;
use std::path::Path;

// TODO: Use our own library's formatting instead of expect()!
// We need either:
// 1. a `Result` function which means “our error handling strategy is panic()”,
//    without the specific meanings "todo" or "unreachable",
// 2. a function that panics, with the provided message and error.

fn main() {
    let part_directory = Path::new(&env!("CARGO_MANIFEST_DIR")).join("../doc-example-parts/");
    rerun_if_path_changed(&part_directory);

    let mut output_file = fs::File::create(
        Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR should be set"))
            .join("doc-example-parts.rs"),
    )
    .expect("open output file");

    for entry in fs::read_dir(part_directory).expect("open part directory") {
        let entry = entry.expect("read part directory");

        let part_failing_path = entry.path();
        let failing_name = entry
            .file_name()
            .into_string()
            .expect("file name should be UTF-8");
        let Some(part_name_stem) = failing_name.strip_suffix("_failing.rs") else {
            continue;
        };

        let mut part_prefix_path = part_failing_path.clone();
        part_prefix_path.set_file_name(format!("{part_name_stem}_prefix.rs"));

        let part_failing_contents =
            unhide(fs::read_to_string(&part_failing_path).expect("reading part _failing file"));
        let part_prefix_contents =
            unhide(fs::read_to_string(&part_prefix_path).expect("reading part _prefix file"));
        rerun_if_path_changed(&part_failing_path);
        rerun_if_path_changed(&part_prefix_path);

        // TODO: Implement processing of "#" prefixes so that we can have hidden lines.

        writeln!(
            output_file,
            "pub fn {part_name_stem}() {{\n\
            {part_prefix_contents}\n\
            {part_failing_contents}\n\
            }}"
        )
        .expect("write output file");
    }

    output_file.flush().expect("flush output file");
}

/// Remove all "# " prefixes in order to emulate the rustdoc feature of hiding lines.
fn unhide(input: String) -> String {
    let mut output = String::new();
    for line in input.lines() {
        if let Some(hidden) = line.strip_prefix("#") {
            output.push_str(hidden);
        } else {
            output.push_str(line);
        }
    }
    output
}

fn rerun_if_path_changed(path: &Path) {
    println!(
        "cargo::rerun-if-changed={}",
        path.to_str().expect("all paths must be UTF-8")
    );
}
