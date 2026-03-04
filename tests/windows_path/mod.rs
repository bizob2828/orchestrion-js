use crate::common::*;
use orchestrion_js::*;
use std::path::PathBuf;

#[test]
fn windows_path() {
    transpile_and_test_with_path(
        file!(),
        false,
        Config::new_single(InstrumentationConfig::new(
            "fetch_decl",
            windows_module_matcher(),
            FunctionQuery::function_declaration("fetch", FunctionKind::Async),
        )),
        PathBuf::from("lib\\index.mjs"),
    );
}
