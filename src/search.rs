use regex;

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, regex::Error> {
    let re = regex::Regex::new(query)?;
    Ok(contents
        .lines()
        .filter(|line| re.captures(line).is_some())
        .collect())
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Result<Vec<&'a str>, regex::Error> {
    let query = query.to_lowercase();
    let re = regex::Regex::new(&query)?;
    Ok(contents
        .lines()
        .filter(|line| re.captures(&line.to_lowercase()).is_some())
        .collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search("duct", contents).unwrap()
        );
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search("uct", contents).unwrap()
        );
        assert_eq!(
            vec!["safe, fast, productive."],
            search(r"^s", contents).unwrap()
        );
    }

    #[test]
    fn case_insensitive() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive("rUsT", contents).unwrap()
        );
    }
}
