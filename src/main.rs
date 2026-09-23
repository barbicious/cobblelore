use crate::state::State;

mod state;
pub mod renderer;
mod math;

fn main() -> anyhow::Result<()> {
    let state = State::new()?;
    state.run()?;

    Ok(())
}
