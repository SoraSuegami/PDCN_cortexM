#[macro_export]
macro_rules! define_host {
    ({types:[$(($variable:ident,$type:ty)),*],impls:[$(($index:expr,$name:expr,$func:ident,($signature:expr))),*]}) => {
        use core::marker::PhantomData;
        use core::fmt;
        use fmt::{Display,Formatter};
        use alloc::vec::Vec;
        use alloc::boxed::Box;
        use wasmi::{ModuleRef, Externals, ModuleImportResolver, RuntimeArgs, RuntimeValue, ValueType, Signature, Trap, FuncRef, GlobalRef, GlobalDescriptor, MemoryRef, MemoryDescriptor, MemoryInstance, TableRef, TableDescriptor, Error, FuncInstance, HostError as HostErr};
        use wasmi::memory_units::Pages;
        use common::pdcn_systems::crypto::{Sha256Base, Sha256, Signature as CryptoSignature, SignatureBase};
        use pdcn_wasm_management::traits::{ManagementHelper,　Host, HostBuilder、Storage};


        #[derive(Debug)]
        struct HostError(Vec<u8>);

        impl Display for HostError {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                Ok(())
            }
        }

        impl HostErr for HostError {}
        pub struct HostType<Hash:Sha256, Sign:CryptoSignature> {
            $($variable:$type,)*
            id:ModuleId<<Hash as Sha256>::Hasher>,
            memory:MemoryRef,
            hash:PhantomData<Hash>,
            sign:PhantomData<Sign>,
        }

        impl<Hash, Sign> Host for HostType<Hash, Sign> 
            where
                Hash:Sha256,
                Sign:CryptoSignature
        {
            type Hash = <Hash as Sha256>::Hasher;

            fn get_memory(&self) -> Option<MemoryRef> {
                Some(self.memory.clone())
            }
        }

        impl<Hash, Sign> Externals for HostType<Hash, Sign> 
            where
                Hash:Sha256,
                Sign:CryptoSignature    
        {
            fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
                match index {
                    $(
                        $index => {
                            $func(&mut self, index, args)
                        },
                    )*
                    _ => panic!("Unimplemented function at {}", index),
                }
            }
        }

        impl<Hash, Sign> HostType<Hash, Sign> 
            where
                    Hash:Sha256,
                    Sign:CryptoSignature
        {
            //const MEMORYSIZE:usize = 65536;

            fn new
            (
                $($variable:$type,)*
                id:ModuleId<<Self as Host>::Hash>,
                mem:MemoryRef
            )-> Self {
                Self {
                    $($variable:$variable,)*
                    id: id,
                    memory: mem,
                    hash:PhantomData,
                    sign:PhantomData
                }
            }

            fn get_signature(&self, index: usize) -> Signature {
                match index {
                    $(
                        $index => Signature::new($signature),
                    )*
                    _ => panic!("Unimplemented function at {}", index)
                }
            }
        }

        impl<Hash, Sign> ModuleImportResolver for HostType<Hash, Sign>
            where
                    Hash:Sha256,
                    Sign:CryptoSignature
        {
            fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
                let index = match field_name {
                    $(
                        $name => $index,
                    )*
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

        
        struct Builder<'a,Hash:Sha256,Sign:CryptoSignature> {
            //$($variable:Option<&'a $type>,)*
            id:Option<&'a ModuleId<<Hash as Sha256>::Hasher>>,
            memory:Option<&'a MemoryRef>,
            sign:PhantomData<Sign>
        }

        impl<'a,Hash:Sha256,Sign:CryptoSignature> HostBuilder for Builder<'a,Hash,Sign> {
            fn new() -> Self {
                Self {
                    //$($variable:None,)*
                    id:None,
                    memory:None,
                }
            }

            fn module_id(self, id:& [u8]) -> Self {
                Self {
                    //$($variable: self.$variable,)*
                    id: Some(id),
                    memory: self.memory
                }
            }
        
            fn memory(self, mem:& MemoryRef) -> Self {
                Self {
                    //$($variable: self.$variable,)*
                    id: self.id,
                    memory: Some(mem)
                }
            }
        
            fn build(self) -> HostType<Hash,Sign> {
                let empty_slice:[u8;0] = [];
                let slice:&[u8] = match self.id {
                    Some(i) => i,
                    None => &empty_slice
                };
                let id = ModuleId::<<Hash as Sha256>::Hasher>::from(slice);
                let empty_mem = MemoryInstance::alloc(Pages(0), Some(Pages(Self::MEMORYSIZE))).unwrap();
                let mem = match self.memory {
                    Some(m) => m,
                    None => &empty_mem
                };
                HostType::new(
                    $(*$variable,)*
                    id,
                    mem.clone()
                )
            }
        }



    }
}
/*
#[macro_export]
macro_rules! define_host_impl {
    ($(($index:expr,$name:expr,$func:expr,($signature:expr))),*) => {

        impl<Hash:Sha256, Sign:CryptoSignature> Externals for HostType<Hash, Sign> {
            fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
                match index {
                    $(
                        $index => {
                            $func(&mut self, index, args)
                        },
                    )*
                    _ => panic!("Unimplemented function at {}", index),
                }
            }
        }

        impl<Hash:Sha256, Sign:CryptoSignature> HostType<Hash, Sign> {
            //const MEMORYSIZE:usize = 65536;

            fn get_signature(&self, index: usize) -> Signature {
                match index {
                    $(
                        $index => Signature::new($signature),
                    )*
                    _ => panic!("Unimplemented function at {}", index)
                }
            }
        }

        impl<Hash:Sha256, Sign:CryptoSignature> ModuleImportResolver for HostType<Hash, Sign> {
            fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
                let index = match field_name {
                    $(
                        $name => $index,
                    )*
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
    };
}
*/