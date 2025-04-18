use anyhow::Result;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Disponibility {
    pub login: String,

    pub nombre_de_shifts_souhaite: u8,

    pub lundi_midi: String,
    pub lundi_soir: String,
    pub mardi_midi: String,
    pub mardi_soir: String,
    pub mercredi_midi: String,
    pub mercredi_soir: String,
    pub jeudi_midi: String,
    pub jeudi_soir: String,
    pub vendredi_midi: String,
    pub vendredi_soir: String,
    pub samedi_midi: String,
    pub samedi_soir: String,
    pub dimanche_midi: String,
    pub dimanche_soir: String,
}

pub fn disponibilities() -> Result<Vec<Disponibility>> {
    csv::Reader::from_reader(File::open("disponibility.csv")?)
        .deserialize()
        .try_fold(Vec::new(), |mut acc, elem| {
            let mut disponibility: Disponibility = elem?;

            disponibility.login = disponibility.login.trim().to_lowercase();

            acc.push(disponibility);
            Ok(acc)
        })
}
