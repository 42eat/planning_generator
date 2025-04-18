pub const SHIFT_COUNT: usize = 14;

pub const INCOMPATIBLE_ELEMENTS: &[&str] = &[];

pub const SHIFT: [&str; SHIFT_COUNT] = [
    "lundi_midi",
    "lundi_soir",
    "mardi_midi",
    "mardi_soir",
    "mercredi_midi",
    "mercredi_soir",
    "jeudi_midi",
    "jeudi_soir",
    "vendredi_midi",
    "vendredi_soir",
    "samedi_midi",
    "samedi_soir",
    "dimanche_midi",
    "dimanche_soir",
];

pub type Score = f32;

pub const NEED_BARTENDER: [bool; SHIFT_COUNT] = [
    true, // Lundi midi
    true, // Lundi soir
    true, // Mardi midi
    true, // Mardi soir
    true, // Mercredi midi
    true, // Mercredi soir
    true, // Jeudi midi
    true, // Jeudi soir
    true, // Vendredi midi
    true, // Vendredi soir
    true, // Samedi midi
    true, // Samedi soir
    true, // Dimanche midi
    true, // Dimanche soir
];

pub const MEMBER_COUNT: [usize; SHIFT_COUNT] = [
    5, // Lundi midi
    3, // Lundi soir
    5, // Mardi midi
    3, // Mardi soir
    5, // Mercredi midi
    3, // Mercredi soir
    5, // Jeudi midi
    3, // Jeudi soir
    5, // Vendredi midi
    3, // Vendredi soir
    5, // Samedi midi
    3, // Samedi soir
    5, // Dimanche midi
    3, // Dimanche soir
];

pub const MIN_MEMBER_COUNT: [usize; SHIFT_COUNT] = [
    4, // Lundi midi
    2, // Lundi soir
    4, // Mardi midi
    2, // Mardi soir
    4, // Mercredi midi
    2, // Mercredi soir
    4, // Jeudi midi
    2, // Jeudi soir
    4, // Vendredi midi
    2, // Vendredi soir
    4, // Samedi midi
    2, // Samedi soir
    4, // Dimanche midi
    2, // Dimanche soir
];

pub const AVAILABILITY_SCORE_BASE_IMPACT: Score = 1.;

pub const PREVIOUS_SHIFT_IMPACT_MULTIPLIER: Score = 20.;
pub const SHIFT_IMPACT_MULTIPLIER: Score = 1.5;
pub const SHIFT_IMPACT_ADDER: Score = 10000.;

pub const SHIFT_IMPACT_IF_NECESSARY_MULTIPLIER: Score = 1.5;
pub const _MAX_SCORE_TOP_SCORE_RATIO: Score = 0.5;

pub const ACCEPTABLE_NB_OF_SHIFT_ABOVE_WISH: u8 = 1;
