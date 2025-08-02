use std::env;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const OUTPUT: &str = "env_temp.sh";
const LEPTOS_OUTPUT_NAME: &str = "LEPTOS_OUTPUT_NAME";

pub struct Env;

impl Env {
    pub async fn setup() {
        let leptos_output_name = env::var(LEPTOS_OUTPUT_NAME).unwrap();
        let mut file = File::create(OUTPUT).await.unwrap();
        let line = format!("{LEPTOS_OUTPUT_NAME}={leptos_output_name}");
        file.write_all(line.as_bytes()).await.unwrap();
        unsafe {
            env::set_var(LEPTOS_OUTPUT_NAME, leptos_output_name);
        }
    }
}
