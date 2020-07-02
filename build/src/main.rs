use std::fs;
use std::path::Path;
use std::io::{Write};
use std::os::unix::process::CommandExt;
use id::ModuleId;
use pdcn_system_crypto::Sha256Base;
use sha2::{Sha256 as ExSha256, Digest};
use core::str::from_utf8;

struct Sha256([u8;32]);

impl AsRef<[u8]> for Sha256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Sha256Base for Sha256 {
    const HASH_SIZE: usize = 32;
    type Output = [u8;32];
    fn hash(seed:&[u8]) -> Self::Output {
        let mut hasher = ExSha256::new();
        hasher.update(seed);
        let result = hasher.finalize();
        result.into()
    }
}

//#[tokio::main]
fn main(){
    const WASM_FOLDER:&str = "../wasm_apps";
    const CARGO_FILE_PATH:&str = "Cargo.toml";
    const DEFINE_FILE_PATH:&str = "../common/src/define.rs";
    //let current = env::current_dir().unwrap(); 
    //let wasm_relative = Path::new(WASM_FOLDER);
    let wasm_path_buf = fs::canonicalize(WASM_FOLDER).unwrap();
    let wasm_path = wasm_path_buf.as_path();
    if wasm_path.is_dir() {
        fs::remove_file(DEFINE_FILE_PATH).unwrap();
        let mut defines = fs::OpenOptions::new().append(true).create(true).open(DEFINE_FILE_PATH).unwrap();
        defines.write(b"use crate::define_wasm;\n").unwrap();
        defines.write(b"define_wasm!(").unwrap();
        let wasm_dirs = fs::read_dir(wasm_path).unwrap().map(|folder| folder.unwrap()).filter(|dir:&fs::DirEntry|{
            let dir_path = dir.path();
            dir_path.is_dir() && fs::read(dir_path.join(CARGO_FILE_PATH).as_path()).is_ok()
        }).collect::<Vec<fs::DirEntry>>();
        let size = wasm_dirs.len();
        for (i,entry) in wasm_dirs.into_iter().enumerate() {
            let dir_path = entry.path();
            let dir_str = dir_path.file_name().unwrap().to_str().unwrap();
            let path_buf = wasm_path.join("target/wasm32-unknown-unknown/release/".to_string()+dir_str+".wasm");
            let wasm_path = path_buf.as_path();
            let bytes = fs::read(wasm_path).unwrap();
            let module_id = ModuleId::<Sha256>::from(&bytes[..]);
            defines.write(b"(").unwrap();
            defines.write(format!("{:?}",module_id.as_slice()).as_bytes()).unwrap();
            defines.write(b",").unwrap();
            defines.write(format!("{:?}",&bytes[..]).as_bytes()).unwrap();
            defines.write(b",").unwrap();
            defines.write(bytes.len().to_string().as_bytes()).unwrap();
            defines.write(b",").unwrap();
            defines.write(dir_str.as_bytes()).unwrap();
            defines.write(b")").unwrap();
            if i+1!=size {
                defines.write(b",").unwrap();
            }
        }
        defines.write(b");").unwrap();
    }
}
