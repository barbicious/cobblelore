use crate::state::State;

mod state;
mod renderer;
mod math;
mod level;
mod object_name;
mod keyboard;

fn main() -> anyhow::Result<()> {
    let state = State::new()?;
    state.run()?;

    Ok(())
}
