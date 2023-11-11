use silica_engine::prelude::*;

use godot::{engine::Image, prelude::*};
struct SilicaExtension;
#[gdextension]
unsafe impl ExtensionLibrary for SilicaExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct GDWorld {
    world: World,
    #[base]
    node: Base<Node>,
}

#[godot_api]
impl NodeVirtual for GDWorld {
    fn init(node: Base<Node>) -> Self {
        godot_print!("tick");
        let world = World::new(256, 256);
        let image = Image::new();

        GDWorld { world, node: node }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.world.running {
            self.world.tick();
        }

        // create image and edit pixels based on world.world.particle value
    }
}

impl GDWorld {
    pub fn get_data(&self) -> PackedByteArray {
        let mut data = PackedByteArray::new();
        data.resize(self.world.width as usize * self.world.height as usize * 4);

        for x in 0..self.world.width as usize {
            for y in 0..self.world.height as usize {
                let index = (x + y * self.world.width as usize) * 4;
                let particle = self.world.get_particle(x as i32, y as i32);
                let color = variant_type(particle.variant).color;
                data.set(index, color.0);
                data.set(index + 1, color.1);
                data.set(index + 2, color.2);
                data.set(index + 3, 255);
            }
        }
        data
    }
}
