use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("../config/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
}
