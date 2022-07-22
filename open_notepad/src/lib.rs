use egnitely_client::egnitely_fn;
use serde_json::{Value, json};
use std::os::windows::process::CommandExt;
use std::process::Command;

egnitely_fn!(open_notepad);

async fn open_notepad(input:Value) -> Value {
    const DETACHED_PROCESS: u32 = 0x00000008;
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "notepad.exe"])
            .creation_flags(DETACHED_PROCESS)
            .spawn()
            .expect("failed to execute process");
    }
    json!(input)
}
