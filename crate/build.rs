fn main() {
    let proto_root = "proto";
    let output_dir = "src/protos";
    let proto_filename = "service.proto";

    println!("cargo:rerun-if-changed={}/{}", proto_root, proto_filename);
    let customisations = None;
    protoc_grpcio::compile_grpc_protos(
        &[proto_filename],
        &[proto_root],
        &output_dir,
        customisations,
    )
    .expect("Failed to compile gRPC definitions!");
}