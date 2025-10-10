use cairo_lang_casm::builder::CasmBuilder;
use cairo_lang_casm::casm_build_extend;
use cairo_lang_sierra::extensions::sha256::Sha256ConcreteLibfunc;

use super::{CompiledInvocation, CompiledInvocationBuilder, InvocationError};
use crate::invocations::add_input_variables;

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
    let [sha256, state, message] = builder.try_get_single_cells()?;

    let mut casm_builder = CasmBuilder::default();
    add_input_variables! {casm_builder,
        buffer(8) state;
        buffer(16) message;
        buffer(31) sha256;
    };
    // Write state (8 words) and message (16 words) to the builtin segment as felts.
    for _ in 0..8 {
        casm_build_extend! {casm_builder,
            tempvar tmp = *(state++);
            assert tmp = *(sha256++);
        };
    }
    for _ in 0..16 {
        casm_build_extend! {casm_builder,
            tempvar tmp = *(message++);
            assert tmp = *(sha256++);
        };
    }
    // Allocate output (8 words) and write it to the builtin segment.
    casm_build_extend! {casm_builder,
        tempvar output;
        const state_size = 8;
        hint AllocConstantSize { size: state_size } into { dst: output };
    };
    for _ in 0..8 {
        casm_build_extend! {casm_builder,
            tempvar tmp = *(sha256++);
            assert tmp = *(output++);
        };
    }
    Ok(builder.build_from_casm_builder(
        casm_builder,
        [("Fallthrough", &[&[sha256], &[output]], None)],
        Default::default(),
    ))
}
