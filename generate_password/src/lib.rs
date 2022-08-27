use egnitely_client::egnitely_fn;
use serde::Deserialize;
use serde_json::{Value, json};
use passwords::PasswordGenerator;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

#[derive(Deserialize, Debug)]
struct PasswordGeneratorParams {
    length: u32,
    numbers: bool,
    lowercase_letters: bool,
    uppercase_letters: bool,
    symbols: bool,
    spaces: bool,
}

egnitely_fn!(generate_password);

async fn generate_password(input:Value) -> Value {

    print!("Received Data: {:?}",input.clone());
    let data:PasswordGeneratorParams = serde_json::from_value(input["data"].clone()).unwrap();
    let len = usize::try_from(data.length).unwrap();
    
    let pg = PasswordGenerator {
        length: len,
        numbers: data.numbers,
        lowercase_letters: data.lowercase_letters,
        uppercase_letters: data.uppercase_letters,
        symbols: data.symbols,
        spaces: data.spaces,
        exclude_similar_characters: false,
        strict: true,
    };
    let password = pg.generate_one().unwrap();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
    
    json!({
        "message":"Generated new Password",
        "data":{
            "password": password,
        }
    })
}
