use crate::renderer::texture_registry::TextureRegistry;
use crate::state::State;

mod state;
mod renderer;
mod math;
mod level;
mod registry;
mod object_name;

fn main() -> anyhow::Result<()> {
    let state = State::new()?;
    state.run()?;

    Ok(())
}
