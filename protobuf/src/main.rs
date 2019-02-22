extern crate protobuf_codegen_pure;


fn main() {
	protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "../server/rust/server/src/mouse",
        input: &["protos/touchpad.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
        ..Default::default()
        },
    }).expect("protoc");
	
}