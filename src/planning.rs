use crate::consts::*;
use crate::members::{Availability, Member};
use std::cmp::Ordering;

pub type MemberIndex = usize;

#[derive(Debug)]
struct MemberScore {
    availability_score: Score,
    nb_shift_wanted: u8,
    base_score: Score,
    score: Score,
    shifts: [bool; SHIFT_COUNT],
}

pub fn create_planning(members: &[Member]) -> [Vec<MemberIndex>; SHIFT_COUNT] {
    let mut member_scores = member_scores(members);

    let result: [Vec<MemberIndex>; SHIFT_COUNT] = std::array::from_fn(|shift_index| {
        let top_score = recalculate_score(&mut member_scores, shift_index, members);
        let available_members = available_members(members, shift_index, &member_scores, top_score);
        // print_scores(&member_scores, members, &available_members);
        let shift = create_shift(&mut member_scores, available_members, members, shift_index);
        update_shifts(shift_index, &shift, &mut member_scores);
        print_shift(&shift, shift_index, members);
        shift
    });
    print_member_participation(members, &result, &member_scores);
    print_participation_average(&member_scores);
    result
}

fn member_scores(members: &[Member]) -> Vec<MemberScore> {
    members
        .iter()
        .map(|member| {
            let mut availability_score = 0.;
            let mut nb_of_availabilities = 0;
            let score =
                member
                    .availability
                    .iter()
                    .enumerate()
                    .fold(0., |acc, (index, availability)| {
                        if MEMBER_COUNT[index] == 0 {
                            return acc;
                        }

                        match *availability {
                            Availability::Available => {
                                availability_score += 1.;
                                nb_of_availabilities += 1;
                                acc + AVAILABILITY_SCORE_BASE_IMPACT
                            }
                            Availability::AvailableIfNecessary => {
                                availability_score += 0.5;
                                nb_of_availabilities += 1;
                                acc + AVAILABILITY_SCORE_BASE_IMPACT
                            }
                            Availability::NotAvailable => acc,
                        }
                    });
            MemberScore {
                availability_score,
                nb_shift_wanted: member.nb_shift_wanted.min(nb_of_availabilities),
                base_score: score,
                score,
                shifts: [false; SHIFT_COUNT],
            }
        })
        .collect::<Vec<MemberScore>>()
}

fn available_members(
    members: &[Member],
    shift: usize,
    member_scores: &[MemberScore],
    _top_score: Score,
) -> Vec<MemberIndex> {
    members
        .iter()
        .enumerate()
        .filter_map(|(index, member)| {
            if member.availability[shift] == Availability::NotAvailable
                || member_scores[index].score == Score::INFINITY
            {
                //|| member_scores[index].score / top_score > MAX_SCORE_TOP_SCORE_RATIO {
                return None;
            }
            Some(index)
        })
        .collect()
}

fn create_shift(
    member_scores: &mut [MemberScore],
    mut available_members: Vec<MemberIndex>,
    members: &[Member],
    shift: usize,
) -> Vec<MemberIndex> {
    let members_count_goal = MEMBER_COUNT[shift];
    if members_count_goal == 0 {
        return vec![];
    }

    let mut result = vec![];

    available_members.sort_by(|left, right| {
        member_scores[*left]
            .partial_cmp(&member_scores[*right])
            .expect("partial cmp failed")
    });
    let mut has_incompatible_element: bool = false;

    if NEED_BARTENDER[shift] {
        let Some(bartender_index) = available_members
            .iter()
            .enumerate()
            .filter_map(|(index, member)| {
                if !members[*member].is_bartender {
                    return None;
                }
                Some(index)
            })
            .next()
        else {
            println!("No bartenders available for shift {}", SHIFT[shift]);
            return vec![];
        };

        result.push(available_members[bartender_index]);
        available_members.remove(bartender_index);

        if INCOMPATIBLE_ELEMENTS.contains(&members[result[0]].login.as_str()) {
            has_incompatible_element = true;
        }
    }

    for member in available_members.into_iter() {
        if result.len() == members_count_goal {
            return result;
        }
        if INCOMPATIBLE_ELEMENTS.contains(&members[member].login.as_str()) {
            if has_incompatible_element {
                continue;
            }
            has_incompatible_element = true;
        }
        result.push(member);
    }

    if result.len() >= MIN_MEMBER_COUNT[shift] {
        return result;
    }
    println!("Not enough available members for shift {}", SHIFT[shift]);
    vec![]
}

fn recalculate_score(member_scores: &mut [MemberScore], shift: usize, members: &[Member]) -> Score {
    let mut top_score = Score::MIN;
    for (member_score, member) in member_scores.iter_mut().zip(members) {
        top_score = top_score.max(member_score.recalculate_score(
            shift,
            member.availability[shift],
            &member.login,
        ));
    }
    top_score
}

fn update_shifts(shift_index: usize, shift: &[MemberIndex], member_scores: &mut [MemberScore]) {
    for (index, member_score) in member_scores.iter_mut().enumerate() {
        if shift.contains(&index) {
            member_score.shifts[shift_index] = true;
        }
    }
}

impl MemberScore {
    fn recalculate_score(
        &mut self,
        shift: usize,
        availability: Availability,
        _login: &str,
    ) -> Score {
        let mut nb_of_shift: u8 = 0;
        self.score =
            self.shifts
                .iter()
                .take(shift)
                .fold(self.base_score, |acc, shift_participation| {
                    if !*shift_participation {
                        return acc;
                    }

                    nb_of_shift += 1;
                    acc + SHIFT_IMPACT_ADDER
                });
        if shift > 0 && self.shifts[shift - 1] {
            // self.score = Score::INFINITY;
            // return;
            self.score *= PREVIOUS_SHIFT_IMPACT_MULTIPLIER;
        }
        if shift > 1 && self.shifts[shift - 2] {
            self.score *= SHIFT_IMPACT_MULTIPLIER;
        }

        if availability == Availability::AvailableIfNecessary
            || availability == Availability::NotAvailable
        {
            self.score *= SHIFT_IMPACT_IF_NECESSARY_MULTIPLIER;
        }

        if self.nb_shift_wanted == 0 {
            self.score *= (nb_of_shift + 1) as Score;
        } else {
            self.score *= ((nb_of_shift + 1) as Score) / (self.nb_shift_wanted as Score);
        }
        if nb_of_shift >= self.nb_shift_wanted + ACCEPTABLE_NB_OF_SHIFT_ABOVE_WISH
            || availability == Availability::NotAvailable
        {
            let score = self.score;
            self.score = Score::INFINITY;
            return score;
        }
        self.score
    }
}

#[allow(dead_code)]
fn print_scores(
    member_scores: &[MemberScore],
    members: &[Member],
    available_members: &[MemberIndex],
) {
    for (index, member) in members.iter().enumerate() {
        print!("{}({})", member.login, member_scores[index].score);
        if available_members.contains(&index) {
            print!("+")
        }
        if index != members.len() - 1 {
            print!(", ");
        }
    }
    println!();
}

fn print_shift(shift: &[MemberIndex], shift_index: usize, members: &[Member]) {
    let mut message = SHIFT[shift_index].to_string() + ": [";
    for (index, member) in shift.iter().enumerate() {
        message = message + &members[*member].login;
        if index != shift.len() - 1 {
            message += ", ";
        }
    }
    message += "]\n";
    println!("{message}")
}

fn print_member_participation(
    members: &[Member],
    result: &[Vec<MemberIndex>],
    member_scores: &[MemberScore],
) {
    for (index, member) in members.iter().enumerate() {
        if member_scores[index].availability_score == 0. {
            continue;
        }

        let mut participation = 0;
        for shift in result {
            participation += shift.contains(&index) as u32;
        }
        print!(
            "{}({}/{}/{})",
            member.login,
            participation,
            member_scores[index].nb_shift_wanted,
            member_scores[index].availability_score,
        );
        if index != members.len() - 1 {
            print!(", ");
        }
    }
    println!()
}

fn print_participation_average(member_scores: &[MemberScore]) {
    let mut total_number_of_shifts = 0;
    let mut member_count = 0;

    for member_score in member_scores.iter().filter(|e| e.availability_score != 0.) {
        total_number_of_shifts += member_score
            .shifts
            .iter()
            .fold(0, |acc, elem| acc + (*elem as u32));
        member_count += 1;
    }

    let average_participation = total_number_of_shifts as Score / member_count as Score;
    println!("Average participation of {average_participation} shifts");
}

impl Eq for MemberScore {}

impl PartialEq<Self> for MemberScore {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl PartialOrd<Self> for MemberScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}
