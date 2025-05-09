use std::sync::Arc;

use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_compiler::diagnostics::DiagnosticsReporter;
use cairo_lang_semantic::test_utils::setup_test_module;
use cairo_lang_sierra_generator::db::SierraGenGroup;
use cairo_lang_sierra_generator::program_generator::SierraProgramWithDebug;
use cairo_lang_sierra_generator::replace_ids::replace_sierra_ids_in_program;
use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_vm::Felt252;

use crate::{Arg, SierraCasmRunner};

#[test]
pub fn test_long_arguments() {
    let args: Vec<Arg> = (0..32766).map(|_| Arg::Value(Felt252::ZERO)).collect();

    let db = RootDatabase::builder()
        .with_plugin_suite(starknet_plugin_suite())
        .detect_corelib()
        .build()
        .unwrap();

    let cairo_code: String =
        "fn main(args: Array<felt252>) { println!(\"LEN: {}\", args.len()); }".into();
    let test_module = setup_test_module(&db, &cairo_code).unwrap();

    DiagnosticsReporter::stderr()
        .with_crates(&[test_module.crate_id])
        .allow_warnings()
        .ensure(&db)
        .unwrap();

    // Compile to Sierra.
    let SierraProgramWithDebug { program: sierra_program, debug_info: _ } =
        Arc::unwrap_or_clone(db.get_sierra_program(vec![test_module.crate_id]).expect(
            "`get_sierra_program` failed. run with RUST_LOG=warn (or less) to see diagnostics",
        ));
    let sierra_program = replace_sierra_ids_in_program(&db, &sierra_program);
    let runner = SierraCasmRunner::new(
        sierra_program.clone(),
        Some(Default::default()),
        OrderedHashMap::default(),
        None,
    )
    .unwrap();
    let func = runner.find_function("main").unwrap();
    let _ = runner
        .run_function_with_starknet_context(
            func,
            &[Arg::Array(args)],
            Some(u32::MAX as usize),
            Default::default(),
        )
        .unwrap();
}
