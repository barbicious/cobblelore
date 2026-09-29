use crate::state::State;

mod components;
mod keyboard;
mod level;
mod math;
mod object_name;
mod renderer;
mod state;

fn main() -> anyhow::Result<()> {
    let state = State::new()?;
    state.run()?;

    Ok(())
}
