use std::io;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use serde::Serialize;
use handlebars::Handlebars;

#[derive(Debug, Serialize)]
struct Blog {
    title: String,
    desc: String,
    contents: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from pipe");
    input = input.trim().to_string();
    let path = format!("./../build/blog/blogs/{}.md", &input);

    let file = File::open(path).expect("Failed to open");
    let mut buffer = BufReader::new(file);

    let mut first_line = String::new();
    buffer.read_line(&mut first_line).expect("Unable to read line");
    let last_hash = first_line
        .char_indices()
        .skip_while(|&(_, c)| c == '#')
        .next()
        .map_or(0, |(idx, _)| idx);
    let title: String = first_line[last_hash..].trim().into();

    let mut _date = String::new();
    let mut _n = String::new();
    buffer.read_line(&mut _date);
    buffer.read_line(&mut _n);

    let mut desc = String::new();
    buffer.read_line(&mut desc);
    
    let mut file = File::open("./temp.html")?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?.to_string();

    let blog = Blog {
        title,
        desc,
        contents,
    };

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("blog", "./blog.hbs")?;
    println!("{}", handlebars.render("blog", &blog)?);

    Ok(())
}
