use pest::Parser;
use pest_derive::Parser;

use std::env;

#[derive(Parser)]
#[grammar = "semver.pest"]
struct SemverParser;

#[derive(Debug, Default)]
struct Semver<'a> {
    major: usize,
    minor: usize,
    patch: usize,
    pre: Vec<&'a str>,
    build: Vec<&'a str>,
}

fn main() {
    let args = env::args().skip(1);

    for mut arg in args {
        let mut semver = Semver::default();

        arg = arg.trim().to_owned();
        if arg.find(' ').is_some() {
            continue;
        }
        let pairs = {
            let res = SemverParser::parse(Rule::semver, &arg);
            match res {
                Ok(mut pairs) => pairs.next().unwrap(),
                Err(_) => continue,
            }
        };

        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::core => {
                    let mut inner = pair.into_inner();
                    semver.major = inner.next().unwrap().as_str().parse().unwrap();
                    semver.minor = inner.next().unwrap().as_str().parse().unwrap();
                    semver.patch = inner.next().unwrap().as_str().parse().unwrap();
                }
                Rule::pre => {
                    let mut inner = pair.into_inner();
                    loop {
                        let c = inner.next();
                        if c.is_none() {
                            break;
                        }

                        semver.pre.push(c.unwrap().as_str());
                    }
                }
                Rule::build => {
                    let mut inner = pair.into_inner();
                    loop {
                        let c = inner.next();
                        if c.is_none() {
                            break;
                        }
                        semver.build.push(c.unwrap().as_str());
                    }
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }

        println!("{:#?}\n", semver);
    }
}
