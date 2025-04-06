use crate::system::AssetLibrary;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{AlphaMode2d, Anchor, Material2d, Material2dPlugin, MaterialMesh2dBundle};

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
pub struct SwayingMaterial {
	#[uniform(0)]
	pub sway_strength: f32,
	#[uniform(1)]
	pub sway_speed: f32,
	#[uniform(2)]
	pub vertical_influence: f32,
	#[texture(3)]
	#[sampler(4)]
	pub texture: Handle<Image>,
	#[uniform(5)]
	pub time: f32,
}

impl Default for SwayingMaterial {
	fn default() -> Self {
		Self {
			sway_strength: 0.05,
			sway_speed: 1.0,
			vertical_influence: 0.5,
			texture: Handle::default(),
			time: 0.0,
		}
	}
}
impl Material2d for SwayingMaterial {
	fn vertex_shader() -> ShaderRef {
		ShaderRef::Path("shaders/swaying.wgsl".into())
	}

	fn fragment_shader() -> ShaderRef {
		ShaderRef::Path("shaders/swaying.wgsl".into())
	}

	fn alpha_mode(&self) -> AlphaMode2d {
		AlphaMode2d::Blend
	}
}

#[derive(SystemParam)]
pub struct SwayingObjectSpawner<'w, 's> {
	commands: Commands<'w, 's>,
	materials: ResMut<'w, Assets<SwayingMaterial>>,
	meshes: ResMut<'w, Assets<Mesh>>,
	assets: Res<'w, AssetLibrary>,
	image_data: Res<'w, Assets<Image>>,
}

impl SwayingObjectSpawner<'_, '_> {
	fn get_image_size(&self, handle: &Handle<Image>) -> (u32, u32) {
		let image_data = self.image_data.get(handle).unwrap();
		(image_data.width(), image_data.height())
	}

	pub(crate) fn spawn_kelp(&mut self, sprite: impl ToString, position: Vec3) {
		let handle = self.assets.image(sprite);
		let image_size = self.get_image_size(&handle);
		let mesh = self.meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(
			image_size.0 as f32,
			image_size.1 as f32,
		))));

		self.commands.spawn((
			Mesh2d(mesh),
			MeshMaterial2d(self.materials.add(SwayingMaterial {
				texture: handle.clone_weak(),
				..default()
			})),
			Transform::from_translation(position),
			Visibility::Visible,
			Anchor::BottomCenter,
		));
	}
}

fn update_kelp_sway_time(mut materials: ResMut<Assets<SwayingMaterial>>, time: Res<Time>) {
	for (_, material) in materials.iter_mut() {
		material.time = time.elapsed_secs();
	}
}

pub struct SwayingMaterialPlugin;
impl Plugin for SwayingMaterialPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, update_kelp_sway_time)
			.add_plugins(Material2dPlugin::<SwayingMaterial>::default());
	}
}
