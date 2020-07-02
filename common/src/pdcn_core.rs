use pdcn_wasm_management::traits::{ManagementHelper,Host};
use id::ModuleId;
use pdcn_wasm_management::error::ManagerError;
use pdcn_wasm_management::manager::ModuleManager;
use crate::pdcn_systems::crypto::{Sha256Base, Sha256, Signature as CryptoSignature, SignatureBase};
use wasmi::{ModuleRef, Externals, ModuleImportResolver, RuntimeArgs, RuntimeValue, ValueType, Signature, Trap, FuncRef, GlobalRef, GlobalDescriptor, MemoryRef, MemoryDescriptor, MemoryInstance, TableRef, TableDescriptor, Error, FuncInstance, HostError as HostErr};
use wasmi::memory_units::Pages;
use crate::define::bytes_of_id;
use core::marker::PhantomData;
use core::fmt;
use fmt::{Display,Formatter};
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc_cortex_m::CortexMHeap;


#[derive(Debug)]
struct HostError(Vec<u8>);

impl Display for HostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl HostErr for HostError {}
pub struct HostType<Hash:Sha256, Sign:CryptoSignature> {
    id:ModuleId<Hash::Hasher>,
    memory:MemoryRef,
    hash:PhantomData<Hash>,
    sign:PhantomData<Sign>
}

impl<Hash:Sha256, Sign:CryptoSignature> Externals for HostType<Hash, Sign> {
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            0 => {
                let data_ptr = args.nth_checked::<i32>(0).unwrap() as u32;
                let size = args.nth_checked::<i32>(1).unwrap() as usize;
                let new_ptr = args.nth_checked::<i32>(2).unwrap() as u32;
                let data = self.memory.get(data_ptr, size).unwrap();
                self.memory.set(new_ptr, &data[..]).unwrap();
                Ok(None)
            },
            1 => {
                Ok(Some(RuntimeValue::from(Hash::SIZE as i32)))
            },
            2 => {
                let data_ptr = args.nth_checked::<i32>(0).unwrap() as u32;
                let size = args.nth_checked::<i32>(1).unwrap() as usize;
                let new_ptr = args.nth_checked::<i32>(2).unwrap() as u32;
                Hash::hash(&self.memory, data_ptr, size, new_ptr);
                Ok(None)
            },
            3 => {
                Ok(Some(RuntimeValue::from(<Sign::Base as SignatureBase>::SIGNATURE_SIZE as i32)))
            },
            4 => {
                let data_ptr = args.nth_checked::<i32>(0).unwrap() as u32;
                let size = args.nth_checked::<i32>(1).unwrap() as usize;
                let new_ptr = args.nth_checked::<i32>(2).unwrap() as u32;
                Sign::get_signature(&self.memory, data_ptr, size, new_ptr);
                Ok(None)
            }
            _ => panic!("Unimplemented function at {}", index),
        }
    }
}

impl<Hash:Sha256, Sign:CryptoSignature> HostType<Hash, Sign> {
    //const MEMORYSIZE:usize = 65536;

    fn get_signature(&self, index: usize) -> Signature {
        match index {
            0 => Signature::new(&[ValueType::I32,ValueType::I32,ValueType::I32][..], None),
            1 => Signature::new(&[][..], Some(ValueType::I32)),
            2 => Signature::new(&[ValueType::I32,ValueType::I32,ValueType::I32][..], None),
            3 => Signature::new(&[][..], Some(ValueType::I32)),
            4 => Signature::new(&[ValueType::I32,ValueType::I32,ValueType::I32][..], None),
            _ => panic!("Unimplemented function at {}", index)
        }
    }
}

impl<Hash:Sha256, Sign:CryptoSignature> ModuleImportResolver for HostType<Hash, Sign> {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        let index = match field_name {
            "copy" => 0,
            "get_hash_size" => 1,
            "hash" => 2,
            "get_signature_size" => 3,
            "sign" => 4,
            _ => {
                return Err(Error::Host(Box::new(HostError(b"Export not found".to_vec()))))
            }
        };
        
        let signature = self.get_signature(index);

        Ok(FuncInstance::alloc_host(
            signature,
            index,
        ))
    }

    fn resolve_global(&self, field_name: &str, _global_type: &GlobalDescriptor) -> Result<GlobalRef, Error> {
        Err(Error::Host(Box::new(HostError(b"Export not found".to_vec()))))
    }

    fn resolve_memory(&self, field_name: &str, _memory_type: &MemoryDescriptor) -> Result<MemoryRef, Error> {
        Err(Error::Host(Box::new(HostError(b"Export not found".to_vec()))))
    }

    fn resolve_table(&self, field_name: &str, _table_type: &TableDescriptor) -> Result<TableRef, Error> {
        Err(Error::Host(Box::new(HostError(b"Export not found".to_vec()))))
    }
}


impl<Hash:Sha256, Sign:CryptoSignature> Host for HostType<Hash, Sign> {
    type Hash = Hash::Hasher;

    fn new(id:ModuleId<Self::Hash>,mem:MemoryRef)-> Self {
        Self {
            id: id,
            memory: mem,//MemoryInstance::alloc(Pages(0), Some(Pages(Self::MEMORYSIZE))).unwrap(),
            hash:PhantomData,
            sign:PhantomData
        }
    }

    fn get_memory(&self) -> Option<MemoryRef> {
        Some(self.memory.clone())
    }
}

pub struct Helper<H:Sha256Base> {
    hasher:PhantomData<H>
}

impl<H:Sha256Base> Helper<H> {

    fn new() -> Self{
        Self {
            hasher:PhantomData
        }
    }
}

impl<H:Sha256Base> ManagementHelper for Helper<H> {
    type Hash = H;
    const ENTRY_FUNC:&'static str = "main";
    const ENTRY_MEMORY:&'static str = "memory";
    const HOST_MODULE:&'static str = "host";
    const VERIFY_MODULE: &'static str= "ATTESTATION";

    fn bytes_of_id(id:&ModuleId<Self::Hash>) -> Option<&[u8]> {
        bytes_of_id(id)
    }

    /*fn import_module(self,module_id:ModuleId<Self::Hash>, module:ModuleRef) -> Self {
        let mut new_ids:Vec<ModuleId<Self::Hash>> = self.ids;
        let mut new_refs:Vec<ModuleRef> = self.refs;
        new_ids.push(module_id);
        new_refs.push(module);
        Self {
            ids:new_ids,
            refs:new_refs,
            hasher:PhantomData
        }
    }

    fn get_ref_of_id(&self, module_id:&ModuleId<Self::Hash>) -> Option<&ModuleRef> {
        let index = self.ids.iter().enumerate().map(|(index,id)| {
            match id==module_id {
                true => Some(index),
                false => None
            }
        }).take(1).collect::<Vec<Option<usize>>>()[0];
        match index {
            None => None,
            Some(i) => {
                self.refs.get(i)
            }
        }
    }*/
}
