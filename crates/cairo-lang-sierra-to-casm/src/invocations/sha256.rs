use cairo_lang_casm::builder::CasmBuilder;
use cairo_lang_casm::casm_build_extend;
use cairo_lang_sierra::extensions::sha256::Sha256ConcreteLibfunc;

use super::{CompiledInvocation, CompiledInvocationBuilder, InvocationError};
use crate::invocations::{CostValidationInfo, add_input_variables};

/// Builds instructions for Sierra sha256 operations.
pub fn build(
    libfunc: &Sha256ConcreteLibfunc,
    builder: CompiledInvocationBuilder<'_>,
) -> Result<CompiledInvocation, InvocationError> {
    match libfunc {
        Sha256ConcreteLibfunc::Sha256Compress(_) => build_sha256_compress(builder),
    }
}

/// Handles instruction for computing the sha256 compress.
fn build_sha256_compress(
    builder: CompiledInvocationBuilder<'_>,
) -> Result<CompiledInvocation, InvocationError> {
    let [sha256, state, message] = builder.try_get_refs()?;
    let sha256 = sha256.try_unpack_single()?;
    let mut casm_builder = CasmBuilder::default();
    add_input_variables! {casm_builder,
        buffer(32) sha256;
    };
    for cell in &state.cells {
        add_input_variables!(casm_builder, deref cell;);
        casm_build_extend!(casm_builder, assert cell = *(sha256++););
    }
    for cell in &message.cells {
        add_input_variables!(casm_builder, deref cell;);
        casm_build_extend! {casm_builder, assert cell = *(sha256++); };
    }
    casm_build_extend! {casm_builder,
        let output = sha256;
        const output_size = 8;
        let sha256_end = sha256 + output_size;
    };
    Ok(builder.build_from_casm_builder(
        casm_builder,
        [("Fallthrough", &[&[sha256_end], &[output]], None)],
        CostValidationInfo::default(),
    ))
}
