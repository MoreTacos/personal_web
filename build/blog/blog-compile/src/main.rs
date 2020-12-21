use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;
use handlebars::Handlebars;
use glob::glob;
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize, Ord, PartialOrd, Eq, PartialEq)]
struct Blog {
    date: NaiveDate,
    link: String,
    title: String,
}

fn title_date_string<R>(mut rdr: R) -> (String, String)
    where R: BufRead,
{
    let mut first_line = String::new();
    let mut second_line = String::new();
    rdr.read_line(&mut first_line).expect("Unable to read line");
    let last_hash = first_line
        .char_indices()
        .skip_while(|&(_, c)| c == '#')
        .next()
        .map_or(0, |(idx, _)| idx);

    rdr.read_line(&mut second_line).expect("Unable to read line");

    // Trim the leading hashes and any whitespace
    (first_line[last_hash..].trim().into(), second_line[1..second_line.len()-2].trim().into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut blogs: Vec<Blog> = vec![];

    for entry in glob("./**/*.md").expect("failed to read glob") {
        match entry {
            Ok(path) => {
                let path = path.into_os_string().into_string().unwrap();
                let link = String::from(path.split("/").last().unwrap().split(".").nth(0).unwrap());
                let file = File::open(path).expect("Failed to open");
                let buffer = BufReader::new(file);
                let (title, str_date) = title_date_string(buffer);
                let year = str_date.split(" ").last().unwrap();
                let str_month = str_date.split(" ").nth(0).unwrap();
                let month = match str_month {
                    "Sep" => 9,
                    "Oct" => 10,
                    "Nov" => 11,
                    "Dec" => 12,
                    _ => 0,
                };
                let day = str_date.split(" ").nth(1).unwrap().split(",").nth(0).unwrap();
                let date = NaiveDate::from_ymd(
                    year.parse::<i32>().unwrap(), 
                    month, 
                    day.parse::<u32>().unwrap());
                blogs.push(Blog{ date, link, title });
            },
            Err(e) => println!("{:?}", e),
        }
    }

    blogs.sort();
    blogs.reverse();

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("blog-page", "./blog-page.hbs")?;

    println!("{}", handlebars.render("blog-page", &blogs)?);


    Ok(())
}
