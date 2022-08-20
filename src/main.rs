use clap::{App, Arg};

fn main() {
    let app = App::new("up")
        .about("takes in string and returns all characters uppercase")
        .version("v0.1.0")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("put some text here")
                .required(true)
                .min_values(1)
        )
        .get_matches();

    let text = app.values_of_lossy("text").unwrap();
    let mut collection: Vec<String> = vec![];

    for i in text.iter() {
        collection.push(i.to_uppercase())
    }

    println!("{}", collection.join(" "))
}
