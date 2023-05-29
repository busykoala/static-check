use serde_yaml::Value;


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


pub fn validate(resource: Value) {
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
