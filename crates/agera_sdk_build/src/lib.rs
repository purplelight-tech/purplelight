#![feature(decl_macro)]

use agera_sdk_application_descriptor::ApplicationDescriptor;
use maplit::hashmap;

/// Starts the build script. The build script must not define a `main`
/// function and rather write a main action using this macro.
/// 
/// # Syntax
/// 
/// ```
/// agera_sdk_build::start!({
///     // Main action
/// });
/// ```
pub macro start {
    ($start_action:expr) => {
        fn main() {
            ::agera_sdk_build::__bootstrap(std::env::var("OUT_DIR").unwrap().as_ref());
            $start_action;
        }
    },
}

#[doc(hidden)]
pub fn __bootstrap(output_directory: &str) {
    use late_substitution::LateSubstitution;

    let output_directory = std::path::PathBuf::from(output_directory);
    let project_path = std::env::current_dir().unwrap();
    let project_path = project_path.to_str().unwrap();
    let descriptor = ApplicationDescriptor::from_project(project_path).unwrap();

    // Write to {output_directory}/agera_sdk_bootstrap/bootstrap.rs
    let descriptor_rs_path = output_directory.join("agera_sdk_bootstrap/bootstrap.rs");
    std::fs::write(descriptor_rs_path, include_str!("./template_code/bootstrap.rs").late_substitution(hashmap! {
        "id".into() => descriptor.id.clone(),
    })).unwrap();
}