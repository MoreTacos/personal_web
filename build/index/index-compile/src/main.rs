use serde::Deserialize;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use handlebars::Handlebars;

#[derive(Debug, Deserialize)]
struct Index {
    tldr: Option<TldrConfig>,
    now: Option<Vec<ActionConfig>>,
    work: Option<Vec<WorkConfig>>,
    done: Option<Vec<ActionConfig>>,
}

#[derive(Debug, Deserialize)]
struct TldrConfig {
    desc: Option<String>,
    looking: Option<String>,
    github: Option<String>,
    youtube: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ActionConfig {
    high: Option<String>,
    desc: Option<String>,
    link: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WorkConfig {
    name: Option<String>,
    tldr: Option<String>,
    desc: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("./index.toml")?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(&s),
        Err(e) => Err(e),
    };
    let parsed: toml::Value = toml::from_str(&s).unwrap();
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "./index.hbs")?;

    println!("{}", handlebars.render("index", &parsed)?);
    Ok(())
}
