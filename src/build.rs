fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir("src/presentation/grpc/generated")
        .compile(
            &["src/presentation/grpc/proto/promotion_service.proto"],
            &["src/presentation/grpc/proto"],
        )?;

    Ok(())
}
