use amethyst::{
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::camera::Projection,
};
use amethyst_rendy::Camera;

pub struct CameraZoomSystem {
    scale: f32,
}

impl CameraZoomSystem {
    pub fn new() -> CameraZoomSystem {
        CameraZoomSystem { scale: 1. }
    }
}

pub struct ZoomCamera {
    pub dimension: (f32, f32),
}

impl Component for ZoomCamera {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> System<'a> for CameraZoomSystem {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        ReadStorage<'a, ZoomCamera>,
        WriteStorage<'a, Camera>,
    );

    fn run(&mut self, (input, zoom_camera, mut camera): Self::SystemData) {
        if let Some(forward) = input.axis_value("forward") {
            self.scale += forward * 0.01;
            for (zoom, camera) in (&zoom_camera, &mut camera).join() {
                camera.set_projection(Projection::orthographic(
                    -zoom.dimension.0 / 2. * self.scale,
                    zoom.dimension.1 / 2. * self.scale,
                    -zoom.dimension.1 / 2. * self.scale,
                    zoom.dimension.1 / 2. * self.scale,
                    0.1,
                    2000.,
                ));
            }
        }
    }
}
