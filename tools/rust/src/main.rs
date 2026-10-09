use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<_> = env::args_os().skip(1).collect();
    if arguments.len() != 3 {
        return Err("expected: protoc schema-directory output-directory".into());
    }
    let compiler = PathBuf::from(&arguments[0]);
    let schema = PathBuf::from(&arguments[1]);
    let output = PathBuf::from(&arguments[2]);
    fs::create_dir_all(&output)?;
    let descriptor = output.join("descriptor.bin");
    let mut prost = tonic_prost_build::Config::new();
    prost.protoc_executable(compiler);
    tonic_prost_build::configure()
        .out_dir(&output)
        .file_descriptor_set_path(&descriptor)
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .client_mod_attribute(".", "#[cfg(feature = \"grpc\")]")
        .server_mod_attribute(".", "#[cfg(feature = \"grpc\")]")
        .compile_with_config(
            prost,
            &[
                schema.join("rime/text_to_speech.proto"),
                schema.join("rime/speech_to_text.proto"),
                schema.join("google/rpc/status.proto"),
            ],
            &[schema],
        )?;
    pbjson_build::Builder::new()
        .out_dir(&output)
        .register_descriptors(&fs::read(&descriptor)?)?
        .build(&[".rime", ".google.rpc"])?;
    fs::remove_file(descriptor)?;
    Ok(())
}
