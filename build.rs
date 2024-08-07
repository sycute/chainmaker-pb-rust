use std::{fs, path::PathBuf, process::Command};

fn scan_protos(dir: &str) -> Vec<String> {
    let mut protos = Vec::new();

    let read_dir = fs::read_dir(dir).unwrap();

    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let sub_protos = scan_protos(&path.display().to_string());
            protos.extend(sub_protos);
        } else if path.extension().unwrap_or_default() == "proto" {
            protos.push(path.display().to_string());
        }
    }

    protos
}

fn main() {
    // env::set_var("OUT_DIR", "src");
    let out_dir = PathBuf::from("src");

    let protos = [
        scan_protos("proto/accesscontrol"),
        scan_protos("proto/common"),
        scan_protos("proto/discovery"),
        scan_protos("proto/net"),
        scan_protos("proto/store"),
        scan_protos("proto/sync"),
        scan_protos("proto/txpool"),
        scan_protos("proto/consensus"),
        scan_protos("proto/consensus/maxbft"),
        scan_protos("proto/consensus/tbft"),
        scan_protos("proto/consensus/dpos"),
        scan_protos("proto/config"),
        scan_protos("proto/syscontract"),
        scan_protos("proto/txfilter"),
        scan_protos("proto/tee"),
        scan_protos("proto/vm"),
        scan_protos("proto/archivecenter"),
    ]
    .concat();

    tonic_build::configure()
        .out_dir(out_dir.clone())
        .build_client(true)
        .build_server(false)
        .compile(
            scan_protos("proto/api").as_slice(),
            &["googleapis", "proto"],
        )
        .unwrap();

    tonic_build::configure()
        .out_dir(out_dir)
        // .file_descriptor_set_path(out_dir.join("chainmaker_pb_descriptor.bin"))
        .build_client(false)
        .build_server(false)
        .compile(protos.as_slice(), &["proto"])
        .unwrap();

    Command::new("cargo").args(&["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=protos/*.proto");
}
