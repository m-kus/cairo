use crate::define_libfunc_hierarchy;
use crate::extensions::boxing::box_ty;
use crate::extensions::int::unsigned::Uint32Type;
use crate::extensions::lib_func::{
    DeferredOutputKind, LibfuncSignature, OutputVarInfo, ParamSignature, SierraApChange,
    SignatureSpecializationContext,
};
use crate::extensions::utils::fixed_size_array_ty;
use crate::extensions::{
    NamedType, NoGenericArgsGenericLibfunc, NoGenericArgsGenericType, OutputVarReferenceInfo,
    SpecializationError,
};
use crate::ids::{ConcreteTypeId, GenericTypeId};

/// Type representing the sha256 state handle.
#[derive(Default)]
pub struct Sha256Type {}

impl NoGenericArgsGenericType for Sha256Type {
    const ID: GenericTypeId = GenericTypeId::new_inline("Sha256");
    const STORABLE: bool = true;
    const DUPLICATABLE: bool = true;
    const DROPPABLE: bool = true;
    const ZERO_SIZED: bool = false;
}

define_libfunc_hierarchy! {
    pub enum Sha256Libfunc {
        Sha256Compress(Sha256CompressLibfunc),
    }, Sha256ConcreteLibfunc
}

/// Libfunc for the sha256_compress function.
#[derive(Default)]
pub struct Sha256CompressLibfunc {}
impl NoGenericArgsGenericLibfunc for Sha256CompressLibfunc {
    const STR_ID: &'static str = "sha256_compress";

    fn specialize_signature(
        &self,
        context: &dyn SignatureSpecializationContext,
    ) -> Result<LibfuncSignature, SpecializationError> {
        let sha256_ty = context.get_concrete_type(Sha256Type::id(), &[])?;
        let boxed_u32x8_ty = boxed_u32_fixed_array_ty(context, 8)?;
        let boxed_u32x16_ty = boxed_u32_fixed_array_ty(context, 16)?;
        Ok(LibfuncSignature::new_non_branch_ex(
            vec![
                ParamSignature::new(sha256_ty.clone()),
                ParamSignature::new(boxed_u32x8_ty.clone()), // Hasher state
                ParamSignature::new(boxed_u32x16_ty),        // Current block
            ],
            vec![
                OutputVarInfo::new_builtin(sha256_ty, 0),
                OutputVarInfo {
                    ty: boxed_u32x8_ty,
                    ref_info: OutputVarReferenceInfo::Deferred(DeferredOutputKind::Generic),
                }, // New state
            ],
            SierraApChange::Known { new_vars_only: true },
        ))
    }
}

/// Returns `Box<[u32; size]>` according to the given size.
fn boxed_u32_fixed_array_ty(
    context: &dyn SignatureSpecializationContext,
    size: i16,
) -> Result<ConcreteTypeId, SpecializationError> {
    let ty = context.get_concrete_type(Uint32Type::id(), &[])?;
    box_ty(context, fixed_size_array_ty(context, ty, size)?)
}
