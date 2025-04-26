mod disponibility;
use disponibility::disponibilities;

mod bartenders;
use bartenders::bartenders;

use crate::consts::SHIFT_COUNT;
use crate::members::disponibility::Disponibility;
use anyhow::Result;

#[derive(Debug)]
pub struct Member {
    pub login: String,
    pub nb_shift_wanted: u8,
    pub availability: [Availability; SHIFT_COUNT],
    pub is_bartender: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Availability {
    Available,
    AvailableIfNecessary,
    NotAvailable,
}

pub fn members() -> Result<Vec<Member>> {
    let bartenders = bartenders()?;
    let result = disponibilities()?
        .into_iter()
        .map(|disponibility| Member {
            login: disponibility.login.clone(),
            nb_shift_wanted: disponibility.nombre_de_shifts_souhaite,
            is_bartender: bartenders.contains(&disponibility.login),
            availability: disponibility.into(),
        })
        .collect();
    Ok(result)
}

impl From<Disponibility> for [Availability; 14] {
    fn from(disponibility: Disponibility) -> Self {
        [
            disponibility.lundi_midi.into(),
            disponibility.lundi_soir.into(),
            disponibility.mardi_midi.into(),
            disponibility.mardi_soir.into(),
            disponibility.mercredi_midi.into(),
            disponibility.mercredi_soir.into(),
            disponibility.jeudi_midi.into(),
            disponibility.jeudi_soir.into(),
            disponibility.vendredi_midi.into(),
            disponibility.vendredi_soir.into(),
            disponibility.samedi_midi.into(),
            disponibility.samedi_soir.into(),
            disponibility.dimanche_midi.into(),
            disponibility.dimanche_soir.into(),
        ]
    }
}

impl From<&str> for Availability {
    fn from(value: &str) -> Self {
        match value {
            "Disponible" => Availability::Available,
            "Disponible seulement si nécessaire" => Availability::AvailableIfNecessary,
            "Non disponible" => Availability::NotAvailable,
            _ => panic!("From<String> for Availability value: {value}"),
        }
    }
}

impl From<String> for Availability {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<Availability> for &str {
    fn from(value: Availability) -> Self {
        match value {
            Availability::Available => "Disponible",
            Availability::AvailableIfNecessary => "Disponible seulement si nécessaire",
            Availability::NotAvailable => "Non disponible",
        }
    }
}
