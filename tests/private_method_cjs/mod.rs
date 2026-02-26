use crate::common::*;
use orchestrion_js::*;

#[test]
fn private_method_cjs() {
    transpile_and_test(
        file!(),
        false,
        Config::new_single(InstrumentationConfig::new(
            "TestClass:testMe",
            test_module_matcher(),
            FunctionQuery::private_method("TestClass", "testMe", FunctionKind::Async),
        )),
    );
}
