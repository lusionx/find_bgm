use clap::{Command, Arg, ArgMatches};

#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub kwords: Vec<String>,
}

impl Config {
    pub fn from_args(matches: ArgMatches) -> Option<Config> {
        let kw: Vec<&str> = matches.values_of("kwords").unwrap().collect();
         
        Some(Config{
            address: String::from(matches.value_of("address")?),
            kwords: kw.iter().map(|e| String::from(*e)).collect(),
        })       
    }
}

pub fn get_config<'a>() -> Command<'a> {
    Command::new("sbgm")
        .version("0.1.0")
        .author("Lusion <lusionx@gmail.com>")
        .about("Search bangumi resources")
        .arg(
            Arg::new("address")
                .short('d')
                .long("address")
                .value_name("search_address")
                .help("search url root")
                .required(true)
        )
        .arg(
            Arg::new("kwords")
                .short('w')
                .long("kwords")
                .value_name("words")
                .help("keys for search")
                .multiple_occurrences(true)               
        )
    }

pub fn parse() -> Config {
    let config = get_config();
    let matches = config.get_matches();
    Config::from_args(matches).unwrap()
}