#[cfg(test)]
mod tests {
    use crate::api_tests::wait_until_all_tests_complete;

    use self::super::super::{SERVER_PROCESS, SERVER_RUNNING};
    use std::net::TcpStream;
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn run() {
        // clean up the docubase crate to trigger the build file
        Command::new("cargo")
            .arg("clean")
            .arg("-p")
            .arg("docubase")
            .spawn()
            .expect("failed to clean docubase")
            .wait()
            .expect("failed to clean docubase");

        Command::new("echo")
            .arg("starting server")
            .spawn()
            .expect("failed to echo")
            .wait()
            .expect("failed to echo");

        let result = Command::new("cargo")
            .arg("run")
            .env("TEST_MODE", "1")
            .current_dir("../")
            .stdout(Stdio::null())
            .spawn();

        if let Ok(child) = result {
            // Wait for the server to start and bind to the port
            const MAX_RETRIES: u32 = 100;
            const RETRY_DELAY: Duration = Duration::from_millis(1000);
            let port = "3000";
            let address = format!("127.0.0.1:{}", port);

            let mut retries = 0;
            while retries < MAX_RETRIES {
                if TcpStream::connect(&address).is_ok() {
                    // Server is up and running
                    if let Ok(mut state) = SERVER_RUNNING.write() {
                        *state = true;
                    }
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

    #[test]
    fn stop() {
        wait_until_all_tests_complete();

        // wait for a little just in case
        std::thread::sleep(std::time::Duration::from_secs(1));

        if let Ok(mut process) = SERVER_PROCESS.write() {
            if let Some(mut proc) = process.take() {
                proc.kill().unwrap()
            }
        }
    }
}
