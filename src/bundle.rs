use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

pub struct RustymonBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for RustymonBundle {
    fn build(self, _builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        Ok(())
    }
}
