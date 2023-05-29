use serde_yaml::Value;


struct Rule {
    kind: String,
    name: String,
    operator: String,
    path: String,
    expected: String,
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


pub fn validate(resource: Value) {
    // example rule
    let rule = Rule {
        kind: "Deployment".to_string(),
        name: "Pod name in Deployment must be nginx".to_string(),
        operator: "=".to_string(),
        path: "spec.template.spec.containers.name".to_string(),
        expected: "nginx".to_string(),
    };
    let rules = vec![rule];

    let kind_vec: Vec<&str> = "kind".split(".").collect();
    let kind = iter(vec![resource.clone()], kind_vec);
    let kind_str = kind.get(0).unwrap().as_str().map(String::from).unwrap();

    let name_vec: Vec<&str> = "metadata.name".split(".").collect();
    let name = iter(vec![resource.clone()], name_vec);
    let name_str = name.get(0).unwrap().as_str().map(String::from).unwrap();

    for rule in rules {
        if rule.kind == kind_str {
            let rule_path = rule.path.as_str().split(".").collect();
            let values = iter(vec![resource.clone()], rule_path);
            for value in values {
                if rule.operator == "=".to_string() {
                    let value_str = value.as_str().unwrap();
                    if value_str != rule.expected {
                        println!("------------------------------");
                        println!("RULE VIOLATION: {}", rule.name);
                        println!("in {} {}", kind_str, name_str);
                        println!("{} should {} {} but is {}", rule.path, rule.operator, rule.expected, value_str);
                    }
                } else {
                    panic!("Operator not implemented.")
                }
            }
        }
    }
}
