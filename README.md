# Planning generator

## Requirement:
### Member csv file
Create a csv file called `members_list.csv` with the following fields:
- login (Can be anything)
- role (Must be one of ["bartender", "coordinateur", "fondateur", "président"])

### Disponibilty csv file
Create a csv file called `disponibility.csv` with the following fields:
- login (Can be anything)
- nombre_de_shifts_souhaite (Must be u8)
- lundi_midi (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- lundi_soir (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- mardi_midi (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- mardi_soir (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- mercredi_midi (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- mercredi_soir (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- jeudi_midi (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- jeudi_soir (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- vendredi_midi (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- vendredi_soir (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- samedi_midi (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- samedi_soir (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- dimanche_midi (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])
- dimanche_soir (Must be one of ["Disponible", "Non disponible", "Disponible seulement si nécessaire"])


## Parameters:
See src/consts.rs

## Usage:
- Install rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Prepare `disponibility.csv` and `members_list.csv` as described above
- Run `cargo run`

This will output the planning on `stdout` as well as generate `result.csv` with the planning in
csv format
