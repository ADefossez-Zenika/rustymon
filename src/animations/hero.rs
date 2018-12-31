use amethyst::{
    animation::*,
    assets::{Handle, Loader},
    ecs::prelude::World,
    renderer::SpriteRender,
};

pub fn build_animation_control_set(world: &mut World) -> AnimationControlSet<u32, SpriteRender> {
    let mut animation_control_set = AnimationControlSet::<u32, SpriteRender>::default();
    animation_control_set.add_animation(
        0,
        &build_idle_animation(world),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Start,
    );

    animation_control_set.add_animation(
        1,
        &build_go_right_animation(world),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Init,
    );

    animation_control_set.add_animation(
        2,
        &build_go_left_animation(world),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Init,
    );

    animation_control_set.add_animation(
        3,
        &build_go_up_animation(world),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Init,
    );

    animation_control_set.add_animation(
        4,
        &build_go_down_animation(world),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Init,
    );
    animation_control_set
}

fn build_idle_animation(world: &mut World) -> Handle<Animation<SpriteRender>> {
    let idle_sampler_handle = build_idle_sampler(world);
    build_animation_from_sampler(world, idle_sampler_handle)
}

fn build_idle_sampler(world: &mut World) -> Handle<Sampler<SpriteRenderPrimitive>> {
    let loader = world.read_resource::<Loader>();

    let idle_sampler = Sampler {
        input: vec![0.0, 0.1, 0.2, 0.3, 0.4],
        output: vec![
            SpriteRenderPrimitive::SpriteIndex(0),
            SpriteRenderPrimitive::SpriteIndex(1),
            SpriteRenderPrimitive::SpriteIndex(2),
            SpriteRenderPrimitive::SpriteIndex(3),
            SpriteRenderPrimitive::SpriteIndex(0),
        ],
        function: InterpolationFunction::Step,
    };

    loader.load_from_data::<Sampler<SpriteRenderPrimitive>, ()>(
        idle_sampler,
        (),
        &world.read_resource(),
    )
}

fn build_go_right_animation(world: &mut World) -> Handle<Animation<SpriteRender>> {
    let idle_sampler_handle = build_go_right_sampler(world);
    build_animation_from_sampler(world, idle_sampler_handle)
}

fn build_go_right_sampler(world: &mut World) -> Handle<Sampler<SpriteRenderPrimitive>> {
    let loader = world.read_resource::<Loader>();

    let idle_sampler = Sampler {
        input: vec![0.0, 0.1, 0.2, 0.3, 0.4],
        output: vec![
            SpriteRenderPrimitive::SpriteIndex(4),
            SpriteRenderPrimitive::SpriteIndex(5),
            SpriteRenderPrimitive::SpriteIndex(6),
            SpriteRenderPrimitive::SpriteIndex(7),
            SpriteRenderPrimitive::SpriteIndex(4),
        ],
        function: InterpolationFunction::Step,
    };

    loader.load_from_data::<Sampler<SpriteRenderPrimitive>, ()>(
        idle_sampler,
        (),
        &world.read_resource(),
    )
}

fn build_go_left_animation(world: &mut World) -> Handle<Animation<SpriteRender>> {
    let idle_sampler_handle = build_go_left_sampler(world);
    build_animation_from_sampler(world, idle_sampler_handle)
}

fn build_go_left_sampler(world: &mut World) -> Handle<Sampler<SpriteRenderPrimitive>> {
    let loader = world.read_resource::<Loader>();

    let idle_sampler = Sampler {
        input: vec![0.0, 0.1, 0.2, 0.3, 0.4],
        output: vec![
            SpriteRenderPrimitive::SpriteIndex(8),
            SpriteRenderPrimitive::SpriteIndex(9),
            SpriteRenderPrimitive::SpriteIndex(10),
            SpriteRenderPrimitive::SpriteIndex(11),
            SpriteRenderPrimitive::SpriteIndex(8),
        ],
        function: InterpolationFunction::Step,
    };

    loader.load_from_data::<Sampler<SpriteRenderPrimitive>, ()>(
        idle_sampler,
        (),
        &world.read_resource(),
    )
}

fn build_go_up_animation(world: &mut World) -> Handle<Animation<SpriteRender>> {
    let idle_sampler_handle = build_go_up_sampler(world);
    build_animation_from_sampler(world, idle_sampler_handle)
}

fn build_go_up_sampler(world: &mut World) -> Handle<Sampler<SpriteRenderPrimitive>> {
    let loader = world.read_resource::<Loader>();

    let idle_sampler = Sampler {
        input: vec![0.0, 0.1, 0.2, 0.3, 0.4],
        output: vec![
            SpriteRenderPrimitive::SpriteIndex(8),
            SpriteRenderPrimitive::SpriteIndex(9),
            SpriteRenderPrimitive::SpriteIndex(10),
            SpriteRenderPrimitive::SpriteIndex(11),
            SpriteRenderPrimitive::SpriteIndex(8),
        ],
        function: InterpolationFunction::Step,
    };

    loader.load_from_data::<Sampler<SpriteRenderPrimitive>, ()>(
        idle_sampler,
        (),
        &world.read_resource(),
    )
}

fn build_go_down_animation(world: &mut World) -> Handle<Animation<SpriteRender>> {
    let idle_sampler_handle = build_go_down_sampler(world);
    build_animation_from_sampler(world, idle_sampler_handle)
}

fn build_go_down_sampler(world: &mut World) -> Handle<Sampler<SpriteRenderPrimitive>> {
    let loader = world.read_resource::<Loader>();

    let idle_sampler = Sampler {
        input: vec![0.0, 0.1, 0.2, 0.3, 0.4],
        output: vec![
            SpriteRenderPrimitive::SpriteIndex(8),
            SpriteRenderPrimitive::SpriteIndex(9),
            SpriteRenderPrimitive::SpriteIndex(10),
            SpriteRenderPrimitive::SpriteIndex(11),
            SpriteRenderPrimitive::SpriteIndex(8),
        ],
        function: InterpolationFunction::Step,
    };

    loader.load_from_data::<Sampler<SpriteRenderPrimitive>, ()>(
        idle_sampler,
        (),
        &world.read_resource(),
    )
}

/// Build a animation from a sampler and return a handle to that animation.
fn build_animation_from_sampler(
    world: &mut World,
    sampler: Handle<Sampler<SpriteRenderPrimitive>>,
) -> Handle<Animation<SpriteRender>> {
    let ide_animation = Animation::<SpriteRender> {
        nodes: vec![(0, SpriteRenderChannel::SpriteIndex, sampler)],
    };

    let loader = world.read_resource::<Loader>();
    loader.load_from_data::<Animation<SpriteRender>, ()>(ide_animation, (), &world.read_resource())
}
