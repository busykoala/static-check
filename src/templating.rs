use std::io;
use std::process::{Command, Output};
use std::env;
use serde_yaml::{Value,from_str};


pub fn get_templates() -> Result<Vec<String>, io::Error> {
    let pwd = env::var("PWD").map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get PWD."))?;

    let output: Output = Command::new("helm")
        .arg("template")
        .arg("-f")
        .arg(format!("{}/values.yaml", pwd))
        .arg(&pwd)
        .output()?;

    if output.status.success() {
        let raw_templates = String::from_utf8_lossy(&output.stdout);
        let templates: Vec<String> = raw_templates.split("---").map(|s| s.trim().to_owned()).collect();
        Ok(templates)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Helm template command failed."))
    }
}


pub fn deserialize(templates: Vec<String>) -> Vec<Value> {
    templates
        .into_iter()
        .filter_map(|template| {
            let value: Value = from_str(&template).expect("Failed to parse YAML");
            if value.is_null() {
                None
            } else {
                Some(value)
            }
        }).collect()
}
