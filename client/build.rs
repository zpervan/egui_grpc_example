use std::io;

fn main() -> io::Result<()> {
    let parent_dir = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();

    let proto_path = format!("{}{}", parent_dir.to_str().unwrap(), "/proto_messages");
    let proto_file_path = format!("{}{}", proto_path, "/messages.proto");

    tonic_build::configure()
        .out_dir("src/proto")
        .build_server(false)
        .build_client(true)
        .build_transport(false)
        .compile(&[proto_file_path], &[proto_path])
}
