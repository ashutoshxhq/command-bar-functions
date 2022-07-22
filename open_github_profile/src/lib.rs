use egnitely_client::egnitely_fn;
use serde_json::{Value, json};
use std::os::windows::process::CommandExt;
use std::process::Command;

egnitely_fn!(open_github_profile);

async fn open_github_profile(input:Value) -> Value {

    print!("Reciedved Data: {:?}",input);
    const DETACHED_PROCESS: u32 = 0x00000008;
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", &format!("https://github.com/{}", &(input["data"]["username"]))])
            .creation_flags(DETACHED_PROCESS)
            .spawn()
            .expect("failed to execute process");
    }
    json!(input)
}
