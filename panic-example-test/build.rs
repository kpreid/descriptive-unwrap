//! Processes the failing doctest code in `../doc-example-parts/` into functions that can be
//! compiled into the `panic-example-test` binary.
//! This binary is then used to verify that the panic output displayed in the documentation is
//! exactly identical to the actual output.

use std::fs;
use std::io::Write as _;
use std::path::Path;

// TODO: Use our own library's formatting instead of expect()!
// We need a function which means “our error handling strategy is panic()”,
// without the specific meanings "todo" or "unreachable".

fn main() {
    let part_directory = Path::new(&env!("CARGO_MANIFEST_DIR")).join("../doc-example-parts/");

    let mut output_file = fs::File::create(
        Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR should be set"))
            .join("doc-example-parts.rs"),
    )
    .expect("open output file");

    for entry in fs::read_dir(part_directory).expect("open part directory") {
        let entry = entry.expect("read part directory");

        let name = entry
            .file_name()
            .into_string()
            .expect("file name should be UTF-8");
        let Some(part_name_stem) = name.strip_suffix(".rs") else {
            continue;
        };
        let part_contents = fs::read_to_string(entry.path()).expect("reading part file");

        // TODO: Implement processing of "#" prefixes so that we can have hidden lines.

        writeln!(
            output_file,
            "pub fn {part_name_stem}() {{\n\
            {part_contents}\n\
            }}"
        )
        .expect("write output file");
    }

    output_file.flush().expect("flush output file");
}
