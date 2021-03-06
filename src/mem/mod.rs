mod read;
mod index;

use std::marker::PhantomData;
use crate::ffi::*;
pub use crate::ffi::MemoryCbFnc;
pub use crate::mem::index::{IndexSet, IndexMove};
pub use crate::mem::read::{MemIndexWrapper, TypedMemoryReader, TypedMemoryWriter};

pub enum Processor {
    Arm9, Arm7
}

impl Processor {
    fn get_name(&self) -> &str {
        match self {
            Processor::Arm9 => "arm9",
            Processor::Arm7 => "arm7",
        }
    }
}

#[non_exhaustive]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15, CPSR, SPSR,
    SP, // Alias for R13
    LR, // Alias for R14
    PC  // Alias for R15
}

impl Register {
    fn get_name(&self) -> &str {
        match self {
            Register::R0 => "r0",
            Register::R1 => "r1",
            Register::R2 => "r2",
            Register::R3 => "r3",
            Register::R4 => "r4",
            Register::R5 => "r5",
            Register::R6 => "r6",
            Register::R7 => "r7",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
            Register::CPSR => "cpsr",
            Register::SPSR => "spsr",

            Register::SP => "r13",
            Register::LR => "r14",
            Register::PC => "r15"
        }
    }
}

/// Access and manipulate the memory of the emulator.
pub struct DeSmuMEMemory(pub(crate) PhantomData<()>);

impl DeSmuMEMemory {
    /// Allows reading the memory using a &\[u8]-like type. Please note that reading memory always copies.
    ///
    /// Use the [`IndexMove`] trait to access the returned data type.
    ///
    /// At the time of writing there is no syntactic sugar that allows indexing to return non-references,
    /// so we have implemented [`IndexMove`] from https://github.com/rust-lang/rfcs/issues/997.
    ///
    /// # Usage example
    /// ```rs
    /// use rs_desmume::DeSmuMEMemory;
    /// use rs_desmume::mem::index::IndexMove;
    ///
    /// fn example(mem: DeSmuMEMemory) {
    ///     let a: u8 = mem.u8().index_move(123);
    ///     let b: Vec<u8> = mem.u8().index_move(123..456);
    /// }
    /// ```
    pub fn u8(&self) -> MemIndexWrapper<TypedMemoryReader<u8>, u8, false> {
        MemIndexWrapper(TypedMemoryReader(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[u8]-like type.
    ///
    /// Use the [`IndexSet`] and [`IndexMove`] traits to access the returned data type.
    /// At the time of writing there is no syntactic sugar that allows indexing to return non-references,
    /// so we have implemented [`IndexSet`] and [`IndexMove`] from https://github.com/rust-lang/rfcs/issues/997.
    ///
    /// # Usage example
    /// ```rs
    /// use rs_desmume::DeSmuMEMemory;
    /// use rs_desmume::mem::index::{IndexMove, IndexSet};
    ///
    /// fn example(mem: DeSmuMEMemory) {
    ///     let a: u8 = mem.u8_mut().index_move(123);
    ///     let b: Vec<u8> = mem.u8().index_move(100..200);
    ///     mem.u8_mut().index_set(456, &a);
    ///     mem.u8_mut().index_set(500..600, &b);
    /// }
    /// ```
    pub fn u8_mut(&mut self) -> MemIndexWrapper<TypedMemoryWriter<u8>, u8, true> {
        MemIndexWrapper(TypedMemoryWriter(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[u16]-like type. Please note that reading memory always copies.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 2 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn u16(&self) -> MemIndexWrapper<TypedMemoryReader<u16>, u16, false> {
        MemIndexWrapper(TypedMemoryReader(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[u16]-like type.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 2 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn u16_mut(&mut self) -> MemIndexWrapper<TypedMemoryWriter<u16>, u16, true> {
        MemIndexWrapper(TypedMemoryWriter(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[u32]-like type. Please note that reading memory always copies.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 4 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn u32(&self) -> MemIndexWrapper<TypedMemoryReader<u32>, u32, false> {
        MemIndexWrapper(TypedMemoryReader(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[u32]-like type.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 4 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn u32_mut(&mut self) -> MemIndexWrapper<TypedMemoryWriter<u32>, u32, true> {
        MemIndexWrapper(TypedMemoryWriter(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[i8]-like type. Please note that reading memory always copies.
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn i8(&self) -> MemIndexWrapper<TypedMemoryReader<i8>, i8, false> {
        MemIndexWrapper(TypedMemoryReader(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[i8]-like type.
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn i8_mut(&mut self) -> MemIndexWrapper<TypedMemoryWriter<i8>, i8, true> {
        MemIndexWrapper(TypedMemoryWriter(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[i16]-like type. Please note that reading memory always copies.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 2 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn i16(&self) -> MemIndexWrapper<TypedMemoryReader<i16>, i16, false> {
        MemIndexWrapper(TypedMemoryReader(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[i16]-like type.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 2 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn i16_mut(&mut self) -> MemIndexWrapper<TypedMemoryWriter<i16>, i16, true> {
        MemIndexWrapper(TypedMemoryWriter(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[i32]-like type. Please note that reading memory always copies.
    /// The returned type is indexed using normal memory addresses (so the size indexed by MUST be a multiple of 4 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn i32(&self) -> MemIndexWrapper<TypedMemoryReader<i32>, i32, false> {
        MemIndexWrapper(TypedMemoryReader(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[i32]-like type.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 4 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn i32_mut(&mut self) -> MemIndexWrapper<TypedMemoryWriter<i32>, i32, true> {
        MemIndexWrapper(TypedMemoryWriter(self, PhantomData), PhantomData)
    }

    pub fn get_reg(&self, processor: Processor, reg: Register) -> u32 {
        let mut bytes = format!("{}.{}", processor.get_name(), reg.get_name()).into_bytes();
        bytes.push(0);
        let mut cchars = bytes.into_iter().map(|b| b as c_char).collect::<Vec<c_char>>();
        unsafe {
            desmume_memory_read_register(cchars.as_mut_ptr())
        }
    }

    pub fn set_reg(&mut self, processor: Processor, reg: Register, value: u32) {
        let mut bytes = format!("{}.{}", processor.get_name(), reg.get_name()).into_bytes();
        bytes.push(0);
        let mut cchars = bytes.into_iter().map(|b| b as c_char).collect::<Vec<c_char>>();
        unsafe {
            desmume_memory_write_register(
                cchars.as_mut_ptr(), value
            )
        }
    }

    pub fn get_next_instruction(&self) -> u32 {
        unsafe { desmume_memory_get_next_instruction() }
    }

    pub fn set_next_instruction(&mut self, value: u32) {
        unsafe { desmume_memory_set_next_instruction(value) }
    }

    /// Add a memory callback for when the memory at the specified address was changed.
    ///
    /// Setting a callback will override the previously registered one for this address.
    /// Set callback to None, to remove the callback for this address.
    ///
    /// `size` is the maximum size that will be watched. If you set this to 4 for example,
    ///  a range of (address, address + 3) will be monitored.
    pub fn register_write(&mut self, address: u32, size: u16, callback: MemoryCbFnc) {
        unsafe { desmume_memory_register_write(address as c_int, size as c_int, callback) }
    }

    /// Add a memory callback for when the memory at the specified address was read.
    ///
    /// Setting a callback will override the previously registered one for this address.
    /// Set callback to None, to remove the callback for this address.
    ///
    /// `size` is the maximum size that will be watched. If you set this to 4 for example,
    ///  a range of (address, address + 3) will be monitored.
    pub fn register_read(&mut self, address: u32, size: u16, callback: MemoryCbFnc) {
        unsafe { desmume_memory_register_read(address as c_int, size as c_int, callback) }
    }

    /// Add a memory callback for when the memory at the specified address was read.
    ///
    /// Setting a callback will override the previously registered one for this address.
    /// Set callback to None, to remove the callback for this address.
    ///
    /// `size` is the maximum size that will be watched. If you set this to 4 for example,
    ///  a range of (address, address + 3) will be monitored.
    pub fn register_exec(&mut self, address: u32, callback: MemoryCbFnc) {
        unsafe { desmume_memory_register_exec(address as c_int, 2, callback) }
    }
}
