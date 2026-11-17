//! Integration tests for SigmaOS-configuration-file-validation.

use sigmaos_configuration_file_validation_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
