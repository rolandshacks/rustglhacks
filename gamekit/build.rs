extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::io::{Write, self};
use std::process::Command;
use std::{env, fs};
use std::fs::File;
use std::path::Path;

fn generate_opengl_bindings() {

    let out_dir = env::var("OUT_DIR").unwrap();

    // generate opengl bindings

    let mut file = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

}

fn compile_shaders() {
    // compiling SPIR-V shaders

    //let gen_path = &Path::new(&out_dir).join("shaders");

    let gen_path = Path::new("shaders");

    let _ = std::fs::create_dir_all(gen_path);

    for entry in std::fs::read_dir("shaders").unwrap() {

        let entry = entry.unwrap();

        if entry.file_type().unwrap().is_file() {
            let in_path = entry.path();
            let in_file = in_path.to_str().unwrap_or_default();

            //println!("cargo:rerun-if-changed={}", in_file);

            let ext = in_path.extension();
            let ext = match ext {
                Some(s) => {
                    match s.to_str() {
                        Some(s) => s,
                        None => ""
                    }
                }
                None => ""
            };

            if ext == "vert" || ext == "frag" {

                let out_file = format!(
                    "{}/{}.spv",
                    gen_path.to_string_lossy(),
                    in_path.file_name().unwrap().to_string_lossy()
                );

                //println!("cargo:warning=compiling shader {} -> {}", in_file, out_file);

                let _ = fs::remove_file(&out_file);


                /*
                let output = Command::new("glslangValidator")
                    .arg("-G")
                    .arg("-o")
                    .arg(&out_file)
                    .arg(&in_file)
                    .output();
                */

                let output = Command::new("glslc")
                    .arg("--target-env=opengl")
                    .arg("-mfmt=bin")
                    .arg("-o")
                    .arg(&out_file)
                    .arg(&in_file)
                    .output();

                let output = match output {
                    Ok(output) => output,
                    Err(e) => {
                        println!("failed to launch glslangValidator : {}", e);
                        return;
                    }
                };

                if !output.status.success() {
                    io::stdout().write_all(&output.stdout).unwrap();
                    io::stderr().write_all(&output.stderr).unwrap();
                    std::process::exit(output.status.code().unwrap_or_default());
                }

            }
        }

    }

    println!("cargo:rerun-if-changed=shaders");
    println!("cargo:rerun-if-changed=build.rs");

}

fn main() {

    generate_opengl_bindings();
    compile_shaders();

}
