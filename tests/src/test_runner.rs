extern crate libtest_mimic;

use libtest_mimic::{Arguments, Trial};
use tokio::runtime::Runtime;
mod api_tests;
mod json_conversions;

//#[allow(dead_code)]
//mod json_templates;

fn start_server() {
    std::env::set_var("TEST_MODE", "1");
    docubase_pre::main().unwrap();
    api_tests::run_server::run();
}

fn stop_server() {
    std::env::remove_var("TEST_MODE");

    api_tests::run_server::stop();
}

fn main() {
    let args = Arguments::from_args();
    let rt = Runtime::new().unwrap();

    start_server();

    let required_before_api_tests = Trial::test("auth", move || {
        rt.block_on(async {
            api_tests::test_auth::signup().await;
            api_tests::test_auth::login().await;
        });

        Ok(())
    });

    libtest_mimic::run(&args, vec![required_before_api_tests]).exit_code();

    let mut failed_tests = 0;

    let registers = [
        api_tests::test_title::register_tests,
        api_tests::test_project_info::register_tests,
        api_tests::test_project_content::register_tests,
        api_tests::test_project_structure::register_tests,
        api_tests::test_additional_instruction::register_tests,
        api_tests::test_project_abstract::register_tests,
        api_tests::test_project_conclusion::register_tests,
        api_tests::test_project_references::register_tests,
        api_tests::test_project_appendices::register_tests,
    ];

    for registery in registers {
        if failed_tests > 0 {
            stop_server();
            panic!("last test gave an error")
        }

        std::thread::sleep(std::time::Duration::from_secs(2));

        let mut tests = Vec::new();
        registery(&mut tests);

        failed_tests = libtest_mimic::run(&args, tests).num_failed;
    }

    std::thread::sleep(std::time::Duration::from_secs(5));
    stop_server();
}
