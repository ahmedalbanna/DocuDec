use dotenv::dotenv;

use self::super::SERVER_PROCESS;
use std::net::TcpStream;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn run() {
    let result = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("docuapi")
        .env("TEST_MODE", "1")
        .current_dir("../")
        .stdout(std::process::Stdio::null())
        .spawn();

    if let Ok(child) = result {
        dotenv().expect("there is no .env found");
        // Wait for the server to start and bind to the port
        const MAX_RETRIES: u32 = 100;
        const RETRY_DELAY: Duration = Duration::from_millis(1000);
        let port = std::env::var("PORT").expect("there is not PORT in the .env");
        let address = format!("127.0.0.1:{}", port);

        let mut retries = 0;
        while retries < MAX_RETRIES {
            if TcpStream::connect(&address).is_ok() {
                // Server is up and running
                if let Ok(mut process) = SERVER_PROCESS.write() {
                    *process = Some(child);
                }

                return;
            }

            // Wait before retrying
            thread::sleep(RETRY_DELAY);
            retries += 1;
        }

        // If we exhausted retries, fail the test
        panic!(
            "Server did not start within the expected time on port {}",
            port
        );
    } else {
        panic!("Failed to spawn the server process");
    }
}

pub fn stop() {
    if let Ok(mut process) = SERVER_PROCESS.write() {
        if let Some(mut proc) = process.take() {
            proc.kill().unwrap()
        }
    }
}
