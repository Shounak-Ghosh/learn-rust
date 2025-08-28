use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // previously parse_config
    // new functions are expected to never fail --> switching to build instead
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // have to consume args[0]
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {query, file_path, ignore_case})
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    // instead using iterator adapter methods
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
    // instead using iterator adapter methods
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
";
        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}