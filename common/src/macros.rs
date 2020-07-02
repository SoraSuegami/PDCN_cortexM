#[macro_export]
macro_rules! define_wasm {
    ($(($id:expr,$bytes:expr,$size:expr,$key:ident)),*) => {

        use pdcn_system_crypto::Sha256Base;
        use id::ModuleId;
        use alloc::vec::Vec;

        pub fn bytes_of_id<H:Sha256Base>(_id:&ModuleId<H>) -> Option<&[u8]> {
            let id_slice = _id.as_slice();
            $(
                if(id_slice == &$id[..]) {
                    return  Some(&($bytes as [u8;$size]));
                }
            )*
            else {
                return None;
            }
        }

        pub struct WasmIDs;

        impl WasmIDs {
            $(
                pub fn $key<H:Sha256Base>() -> ModuleId<H> {
                    let id:&[u8] = &$id;
                    ModuleId::<H>::from(id)
                }
            )*
        }

        #[derive(Default)]
        pub struct WasmStorage {
            $(pub $key:Vec<u8>,)*
        }
    };
}

