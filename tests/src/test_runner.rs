extern crate libtest_mimic;

use std::process::ExitCode;

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

    test_titles(&args);
    test_project_info(&args);
    test_project_content(&args);

    std::thread::sleep(std::time::Duration::from_secs(5));

    stop_server();
}

fn test_project_info(args: &Arguments) -> ExitCode {
    let mut project_info = Vec::new();

    api_tests::test_project_info::register_tests(&mut project_info);

    libtest_mimic::run(args, project_info).exit_code()
}

fn test_project_content(args: &Arguments) -> ExitCode {
    let mut project_content = Vec::new();

    api_tests::test_project_content::register_tests(&mut project_content);

    libtest_mimic::run(args, project_content).exit_code()
}

fn test_titles(args: &Arguments) -> ExitCode {
    let mut titles = Vec::new();

    api_tests::test_title::register_tests(&mut titles);

    libtest_mimic::run(args, titles).exit_code()
}
