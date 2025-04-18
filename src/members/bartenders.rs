use anyhow::Result;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

pub type Login = String;

#[derive(Debug, Deserialize)]
struct MemberCsv {
    login: String,
    role: String,
}

pub fn bartenders() -> Result<HashSet<Login>> {
    let bartender_roles = ["bartender", "coordinateur", "fondateur", "président"]
        .into_iter()
        .collect::<HashSet<&'static str>>();

    csv::Reader::from_reader(File::open("members_list.csv")?)
        .deserialize()
        .try_fold(HashSet::new(), |mut acc, elem| {
            let member: MemberCsv = elem?;

            let role = member.role.trim().to_lowercase();
            if bartender_roles.contains(role.as_str()) {
                let login = member.login.trim().to_lowercase();
                acc.insert(login);
            }

            Ok(acc)
        })
}
