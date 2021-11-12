use std::env;
use std::fs;
use std::path::Path;

use instruments::{code_generator::to_rust, dynamic_graph::{DynamicGraphBuilder}};

fn main() {
    let source = std::fs::read_to_string("synth.synth").unwrap();
    let builder = DynamicGraphBuilder::default();
    let (rust_source, input_count) = to_rust(&builder, &builder.parse_inner(&source).unwrap());

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("synth.rs");
    fs::write(
        &dest_path,
        &format!(
            "
            use instruments::simd_graph::*;
            use generic_array::typenum::*;
            pub fn build_synth() -> impl Node<Input=U{}, Output=U2> + Clone {{
            {}
        }}
        ",
            input_count, rust_source
        ),
    )
    .unwrap();
    std::process::Command::new("rustfmt")
        .arg(&dest_path)
        .spawn()
        .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=synth.synth");
}
