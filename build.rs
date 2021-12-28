fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("src/protogen")
        .compile(
            &["tonic/examples/proto/helloworld/helloworld.proto"],
            &["tonic/examples/proto/helloworld"],
        )?;

    Ok(())
}
