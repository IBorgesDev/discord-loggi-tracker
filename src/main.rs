use headless_chrome::{Browser, LaunchOptions};
use reqwest::Client;
use std::path::PathBuf;

static URL: &str = "Your package URL";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let browser = Browser::new(LaunchOptions {
        path: Some(PathBuf::from("Path/to/chrome.exe")),
        headless: true,
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;
    tab.navigate_to(URL)?;
    tab.wait_until_navigated()?;


    let js_code = r#"
        document.querySelector('span._2wJeVLDbV6aFRqqnMN0wxr').innerText;
    "#;
    let result = tab.evaluate(js_code, false)?.value.unwrap().as_str().unwrap().to_string();



    let client = Client::new();
    let webhook_url = "Your Discord Webhook URL";
    let payload = serde_json::json!({
  "content": null,
  "embeds": [
    {
      "title": "Status do Envio",
      "description": result,
      "color": null,
      "author": {
        "name": "Loggi Envios",
        "url": URL
      }
    }
  ],
  "attachments": []
});
    client.post(webhook_url)
        .json(&payload)
        .send()?;

    Ok(())
}

// made by IBorgesDev (thats not an actual project lol, i made this in 10 minutes)