use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use crate::consts::*;
use crate::members::{Availability, Member};
use crate::planning::MemberIndex;

pub fn generate_planning_csv(members: &[Member], planning: &[Vec<MemberIndex>]) {
    let mut csv: String = get_csv_header();

    for (member_index, member) in members.iter().enumerate() {
        csv += &member.login;
        for (shift_index, shift) in planning.iter().enumerate() {
            let shift = shift.iter().map(|s| *s).collect::<HashSet<MemberIndex>>();
            let member_availability = member.availability[shift_index];
            if !shift.contains(&member_index) {
                csv = csv + "," + member_availability.into();
                continue;
            }
            match member_availability {
                Availability::Available => csv = csv + "," + "A_Present",
                Availability::AvailableIfNecessary => csv = csv + "," + "AIF_Present",
                Availability::NotAvailable => {
                    panic!("Member {} was selected for shift {} but is not available",
                           member.login, SHIFT[shift_index]);
                },
            }
        }
        csv += "\n";
    }

    File::create("result.csv").expect("Failed to create result.csv file")
        .write_all(csv.as_bytes()).expect("Failed to write to result.csv file");
}

fn get_csv_header() -> String {
    "login".to_string()
        + &SHIFT.iter().fold("".to_string(), |acc, shift| acc + "," + shift)
        + "\n"
}
