//! Offsets and sizes of various structs in wasmtime-runtime's vmcontext
//! module.

use cranelift_codegen::ir;
use cranelift_entity::EntityRef;
use cranelift_wasm::{
    DefinedGlobalIndex, DefinedMemoryIndex, DefinedTableIndex, FuncIndex, GlobalIndex, MemoryIndex,
    SignatureIndex, TableIndex,
};
use module::Module;

/// This class computes offsets to fields within `VMContext` and other
/// related structs that JIT code accesses directly.
pub struct VMOffsets {
    /// The size in bytes of a pointer on the target.
    pub pointer_size: u8,
    /// The number of signature declarations in the module.
    pub num_signature_ids: u64,
    /// The number of imported functions in the module.
    pub num_imported_functions: u64,
    /// The number of imported tables in the module.
    pub num_imported_tables: u64,
    /// The number of imported memories in the module.
    pub num_imported_memories: u64,
    /// The number of imported globals in the module.
    pub num_imported_globals: u64,
    /// The number of defined tables in the module.
    pub num_defined_tables: u64,
    /// The number of defined memories in the module.
    pub num_defined_memories: u64,
    /// The number of defined globals in the module.
    pub num_defined_globals: u64,
}

impl VMOffsets {
    /// Return a new `VMOffsets` instance, for a given pointer size.
    pub fn new(pointer_size: u8, module: &Module) -> Self {
        Self {
            pointer_size,
            num_signature_ids: module.signatures.len() as u64,
            num_imported_functions: module.imported_funcs.len() as u64,
            num_imported_tables: module.imported_tables.len() as u64,
            num_imported_memories: module.imported_memories.len() as u64,
            num_imported_globals: module.imported_globals.len() as u64,
            num_defined_tables: module.table_plans.len() as u64,
            num_defined_memories: module.memory_plans.len() as u64,
            num_defined_globals: module.globals.len() as u64,
        }
    }
}

/// Offsets for `VMFunctionImport`.
impl VMOffsets {
    /// The offset of the `body` field.
    #[allow(clippy::erasing_op)]
    pub fn vmfunction_import_body(&self) -> u8 {
        0 * self.pointer_size
    }

    /// The offset of the `vmctx` field.
    #[allow(clippy::identity_op)]
    pub fn vmfunction_import_vmctx(&self) -> u8 {
        1 * self.pointer_size
    }

    /// Return the size of `VMFunctionImport`.
    pub fn size_of_vmfunction_import(&self) -> u8 {
        2 * self.pointer_size
    }
}

/// Offsets for `*const VMFunctionBody`.
impl VMOffsets {
    /// The size of the `current_elements` field.
    #[allow(clippy::identity_op)]
    pub fn size_of_vmfunction_body_ptr(&self) -> u8 {
        1 * self.pointer_size
    }
}

/// Offsets for `VMTableImport`.
impl VMOffsets {
    /// The offset of the `from` field.
    #[allow(clippy::erasing_op)]
    pub fn vmtable_import_from(&self) -> u8 {
        0 * self.pointer_size
    }

    /// The offset of the `vmctx` field.
    #[allow(clippy::identity_op)]
    pub fn vmtable_import_vmctx(&self) -> u8 {
        1 * self.pointer_size
    }

    /// Return the size of `VMTableImport`.
    pub fn size_of_vmtable_import(&self) -> u8 {
        2 * self.pointer_size
    }
}

/// Offsets for `VMTableDefinition`.
impl VMOffsets {
    /// The offset of the `base` field.
    #[allow(clippy::erasing_op)]
    pub fn vmtable_definition_base(&self) -> u8 {
        0 * self.pointer_size
    }

    /// The offset of the `current_elements` field.
    #[allow(clippy::identity_op)]
    pub fn vmtable_definition_current_elements(&self) -> u8 {
        1 * self.pointer_size
    }

    /// The size of the `current_elements` field.
    pub fn size_of_vmtable_definition_current_elements(&self) -> u8 {
        4
    }

    /// Return the size of `VMTableDefinition`.
    pub fn size_of_vmtable_definition(&self) -> u8 {
        2 * self.pointer_size
    }

    /// The type of the `current_elements` field.
    pub fn type_of_vmtable_definition_current_elements(&self) -> ir::Type {
        ir::Type::int(u16::from(self.size_of_vmtable_definition_current_elements()) * 8).unwrap()
    }
}

/// Offsets for `VMMemoryImport`.
impl VMOffsets {
    /// The offset of the `from` field.
    #[allow(clippy::erasing_op)]
    pub fn vmmemory_import_from(&self) -> u8 {
        0 * self.pointer_size
    }

    /// The offset of the `vmctx` field.
    #[allow(clippy::identity_op)]
    pub fn vmmemory_import_vmctx(&self) -> u8 {
        1 * self.pointer_size
    }

    /// Return the size of `VMMemoryImport`.
    pub fn size_of_vmmemory_import(&self) -> u8 {
        2 * self.pointer_size
    }
}

/// Offsets for `VMMemoryDefinition`.
impl VMOffsets {
    /// The offset of the `base` field.
    #[allow(clippy::erasing_op)]
    pub fn vmmemory_definition_base(&self) -> u8 {
        0 * self.pointer_size
    }

    /// The offset of the `current_length` field.
    #[allow(clippy::identity_op)]
    pub fn vmmemory_definition_current_length(&self) -> u8 {
        1 * self.pointer_size
    }

    /// The size of the `current_length` field.
    pub fn size_of_vmmemory_definition_current_length(&self) -> u8 {
        4
    }

    /// Return the size of `VMMemoryDefinition`.
    pub fn size_of_vmmemory_definition(&self) -> u8 {
        2 * self.pointer_size
    }

    /// The type of the `current_length` field.
    pub fn type_of_vmmemory_definition_current_length(&self) -> ir::Type {
        ir::Type::int(u16::from(self.size_of_vmmemory_definition_current_length()) * 8).unwrap()
    }
}

/// Offsets for `VMGlobalImport`.
impl VMOffsets {
    /// The offset of the `from` field.
    #[allow(clippy::erasing_op)]
    pub fn vmglobal_import_from(&self) -> u8 {
        0 * self.pointer_size
    }

    /// Return the size of `VMGlobalImport`.
    #[allow(clippy::identity_op)]
    pub fn size_of_vmglobal_import(&self) -> u8 {
        1 * self.pointer_size
    }
}

/// Offsets for `VMGlobalDefinition`.
impl VMOffsets {
    /// Return the size of `VMGlobalDefinition`.
    pub fn size_of_vmglobal_definition(&self) -> u8 {
        8
    }
}

/// Offsets for `VMSharedSignatureIndex`.
impl VMOffsets {
    /// Return the size of `VMSharedSignatureIndex`.
    pub fn size_of_vmshared_signature_index(&self) -> u8 {
        4
    }
}

/// Offsets for `VMCallerCheckedAnyfunc`.
impl VMOffsets {
    /// The offset of the `func_ptr` field.
    #[allow(clippy::erasing_op)]
    pub fn vmcaller_checked_anyfunc_func_ptr(&self) -> u8 {
        0 * self.pointer_size
    }

    /// The offset of the `type_index` field.
    #[allow(clippy::identity_op)]
    pub fn vmcaller_checked_anyfunc_type_index(&self) -> u8 {
        1 * self.pointer_size
    }

    /// The offset of the `vmctx` field.
    pub fn vmcaller_checked_anyfunc_vmctx(&self) -> u8 {
        2 * self.pointer_size
    }

    /// Return the size of `VMCallerCheckedAnyfunc`.
    pub fn size_of_vmcaller_checked_anyfunc(&self) -> u8 {
        3 * self.pointer_size
    }
}

/// Offsets for `VMContext`.
impl VMOffsets {
    /// The offset of the `signature_ids` field.
    pub fn vmctx_signature_ids(&self) -> i64 {
        0
    }

    /// The offset of the `tables` field.
    #[allow(clippy::erasing_op)]
    pub fn vmctx_imported_functions(&self) -> i64 {
        self.vmctx_signature_ids()
            + self.num_signature_ids as i64 * i64::from(self.size_of_vmshared_signature_index())
    }

    /// The offset of the `tables` field.
    #[allow(clippy::identity_op)]
    pub fn vmctx_imported_tables(&self) -> i64 {
        self.vmctx_imported_functions()
            + self.num_imported_functions as i64 * i64::from(self.size_of_vmfunction_import())
    }

    /// The offset of the `memories` field.
    pub fn vmctx_imported_memories(&self) -> i64 {
        self.vmctx_imported_tables()
            + self.num_imported_tables as i64 * i64::from(self.size_of_vmtable_import())
    }

    /// The offset of the `globals` field.
    pub fn vmctx_imported_globals(&self) -> i64 {
        self.vmctx_imported_memories()
            + self.num_imported_memories as i64 * i64::from(self.size_of_vmmemory_import())
    }

    /// The offset of the `tables` field.
    pub fn vmctx_tables(&self) -> i64 {
        self.vmctx_imported_globals()
            + self.num_imported_globals as i64 * i64::from(self.size_of_vmglobal_import())
    }

    /// The offset of the `memories` field.
    pub fn vmctx_memories(&self) -> i64 {
        self.vmctx_tables()
            + self.num_defined_tables as i64 * i64::from(self.size_of_vmtable_definition())
    }

    /// The offset of the `globals` field.
    pub fn vmctx_globals(&self) -> i64 {
        self.vmctx_memories()
            + self.num_defined_memories as i64 * i64::from(self.size_of_vmmemory_definition())
    }

    /// Return the size of the `VMContext` allocation.
    #[allow(dead_code)]
    pub fn size_of_vmctx(&self) -> i64 {
        self.vmctx_globals()
            + self.num_defined_globals as i64 * i64::from(self.size_of_vmglobal_definition())
    }

    /// Return the offset from the `signature_ids` pointer to `VMSharedSignatureId` index `index`.
    pub fn index_vmshared_signature_id(&self, index: SignatureIndex) -> i32 {
        assert!((index.index() as u64) < self.num_signature_ids);
        cast::i32(
            index
                .as_u32()
                .checked_mul(u32::from(self.size_of_vmshared_signature_index()))
                .unwrap(),
        )
        .unwrap()
    }

    /// Return the offset from the `imported_functions` pointer to `VMFunctionImport` index `index`.
    pub fn index_vmfunction_import(&self, index: FuncIndex) -> i32 {
        assert!((index.index() as u64) < self.num_imported_functions);
        cast::i32(
            index
                .as_u32()
                .checked_mul(u32::from(self.size_of_vmfunction_import()))
                .unwrap(),
        )
        .unwrap()
    }

    /// Return the offset from the `imported_tables` pointer to `VMTableImport` index `index`.
    pub fn index_vmtable_import(&self, index: TableIndex) -> i32 {
        assert!((index.index() as u64) < self.num_imported_tables);
        cast::i32(
            index
                .as_u32()
                .checked_mul(u32::from(self.size_of_vmtable_import()))
                .unwrap(),
        )
        .unwrap()
    }

    /// Return the offset from the `imported_memories` pointer to `VMMemoryImport` index `index`.
    pub fn index_vmmemory_import(&self, index: MemoryIndex) -> i32 {
        assert!((index.index() as u64) < self.num_imported_memories);
        cast::i32(
            index
                .as_u32()
                .checked_mul(u32::from(self.size_of_vmmemory_import()))
                .unwrap(),
        )
        .unwrap()
    }

    /// Return the offset from the `imported_globals` pointer to `VMGlobalImport` index `index`.
    pub fn index_vmglobal_import(&self, index: GlobalIndex) -> i32 {
        assert!((index.index() as u64) < self.num_imported_globals);
        cast::i32(
            index
                .as_u32()
                .checked_mul(u32::from(self.size_of_vmglobal_import()))
                .unwrap(),
        )
        .unwrap()
    }

    /// Return the offset from the `tables` pointer to `VMTableDefinition` index `index`.
    pub fn index_vmtable_definition(&self, index: DefinedTableIndex) -> i32 {
        assert!((index.index() as u64) < self.num_defined_tables);
        cast::i32(
            index
                .as_u32()
                .checked_mul(u32::from(self.size_of_vmtable_definition()))
                .unwrap(),
        )
        .unwrap()
    }

    /// Return the offset from the `memories` pointer to `VMMemoryDefinition` index `index`.
    pub fn index_vmmemory_definition(&self, index: DefinedMemoryIndex) -> i32 {
        assert!((index.index() as u64) < self.num_defined_memories);
        cast::i32(
            index
                .as_u32()
                .checked_mul(u32::from(self.size_of_vmmemory_definition()))
                .unwrap(),
        )
        .unwrap()
    }

    /// Return the offset from the `globals` pointer to the `VMGlobalDefinition`
    /// index `index`.
    pub fn index_vmglobal_definition(&self, index: DefinedGlobalIndex) -> i32 {
        assert!((index.index() as u64) < self.num_defined_globals);
        cast::i32(
            index
                .as_u32()
                .checked_mul(u32::from(self.size_of_vmglobal_definition()))
                .unwrap(),
        )
        .unwrap()
    }

    /// Return the offset from the `imported_functions` pointer to the
    /// `body` field in `*const VMFunctionBody` index `index`.
    pub fn index_vmfunction_import_body(&self, index: FuncIndex) -> i32 {
        self.index_vmfunction_import(index)
            .checked_add(i32::from(self.vmfunction_import_body()))
            .unwrap()
    }

    /// Return the offset from the `imported_functions` pointer to the
    /// `vmctx` field in `*const VMFunctionBody` index `index`.
    pub fn index_vmfunction_import_vmctx(&self, index: FuncIndex) -> i32 {
        self.index_vmfunction_import(index)
            .checked_add(i32::from(self.vmfunction_import_vmctx()))
            .unwrap()
    }

    /// Return the offset from the `tables` pointer to the `from` field in
    /// `VMTableImport` index `index`.
    pub fn index_vmtable_import_from(&self, index: TableIndex) -> i32 {
        self.index_vmtable_import(index)
            .checked_add(i32::from(self.vmtable_import_from()))
            .unwrap()
    }

    /// Return the offset from the `tables` pointer to the `base` field in
    /// `VMTableDefinition` index `index`.
    pub fn index_vmtable_definition_base(&self, index: DefinedTableIndex) -> i32 {
        self.index_vmtable_definition(index)
            .checked_add(i32::from(self.vmtable_definition_base()))
            .unwrap()
    }

    /// Return the offset from the `tables` pointer to the `current_elements` field in
    /// `VMTableDefinition` index `index`.
    pub fn index_vmtable_definition_current_elements(&self, index: DefinedTableIndex) -> i32 {
        self.index_vmtable_definition(index)
            .checked_add(i32::from(self.vmtable_definition_current_elements()))
            .unwrap()
    }

    /// Return the offset from the `memories` pointer to the `from` field in
    /// `VMMemoryImport` index `index`.
    pub fn index_vmmemory_import_from(&self, index: MemoryIndex) -> i32 {
        self.index_vmmemory_import(index)
            .checked_add(i32::from(self.vmmemory_import_from()))
            .unwrap()
    }

    /// Return the offset from the `memories` pointer to the `vmctx` field in
    /// `VMMemoryImport` index `index`.
    pub fn index_vmmemory_import_vmctx(&self, index: MemoryIndex) -> i32 {
        self.index_vmmemory_import(index)
            .checked_add(i32::from(self.vmmemory_import_vmctx()))
            .unwrap()
    }

    /// Return the offset from the `memories` pointer to the `base` field in
    /// `VMMemoryDefinition` index `index`.
    pub fn index_vmmemory_definition_base(&self, index: DefinedMemoryIndex) -> i32 {
        self.index_vmmemory_definition(index)
            .checked_add(i32::from(self.vmmemory_definition_base()))
            .unwrap()
    }

    /// Return the offset from the `memories` pointer to the `current_length` field in
    /// `VMMemoryDefinition` index `index`.
    pub fn index_vmmemory_definition_current_length(&self, index: DefinedMemoryIndex) -> i32 {
        self.index_vmmemory_definition(index)
            .checked_add(i32::from(self.vmmemory_definition_current_length()))
            .unwrap()
    }

    /// Return the offset from the `imported_globals` pointer to the `from` field in
    /// `VMGlobalImport` index `index`.
    pub fn index_vmglobal_import_from(&self, index: GlobalIndex) -> i32 {
        self.index_vmglobal_import(index)
            .checked_add(i32::from(self.vmglobal_import_from()))
            .unwrap()
    }
}