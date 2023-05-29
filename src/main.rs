mod validator;
mod templating;

use crate::templating::{get_templates, deserialize};
use crate::validator::validate;


fn main() {
    let templates = get_templates().unwrap();
    let resources = deserialize(templates);
    for resource in resources {
        validate(resource);
    }
}
