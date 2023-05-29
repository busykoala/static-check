use std::io;
use std::process::{Command, Output};
use std::env;
use serde_yaml::{Value,from_str};


fn get_templates() -> Result<Vec<String>, io::Error> {
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

fn deserialize(templates: Vec<String>) -> Vec<Value> {
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

fn iter(values: Vec<Value>, path: Vec<&str>) -> Vec<Value> {
    if path.is_empty() {
        return values;
    }
    let (first, others) = path.split_at(1);
    let mut out: Vec<Value> = Vec::new();
    for v in values {
        match v {
            Value::Mapping(r) => {
                if let Some(value) = r.get(first[0]) {
                    out.push(value.clone());
                }
            }
            Value::Sequence(r) => {
                for i in r {
                    if let Some(value) = i.get(first[0]) {
                        out.push(value.clone());
                    }
                }
            }
            _ => {}
        }
    }
    iter(out, others.to_vec())
}

fn validate(resource: Value) {
    let path = "spec.template.spec.containers.name";
    let path_vec: Vec<&str> = path.split(".").collect();
    let kind_vec: Vec<&str> = "kind".split(".").collect();
    let name_vec: Vec<&str> = "metadata.name".split(".").collect();

    let kind = iter(vec![resource.clone()], kind_vec);
    let kind_str = kind.get(0).unwrap().as_str().map(String::from).unwrap();

    let name = iter(vec![resource.clone()], name_vec);
    let name_str = name.get(0).unwrap().as_str().map(String::from).unwrap();

    let path = iter(vec![resource.clone()], path_vec);

    println!("------------------------------");
    println!("{} ({}) has value: {:?}", kind_str, name_str, path);
}

fn main() {
    let templates = get_templates().unwrap();
    let resources = deserialize(templates);
    for resource in resources {
        validate(resource);
    }
}
