mod consts;
mod generate_planning_csv;
mod members;
mod planning;

use members::members;
use planning::create_planning;

use crate::generate_planning_csv::generate_planning_csv;
use anyhow::Result;

fn main() -> Result<()> {
    let members = members()?;
    let planning = create_planning(&members);
    generate_planning_csv(&members, &planning);
    Ok(())
}
