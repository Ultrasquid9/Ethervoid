use imageproc::{
	geometric_transformations::rotate_about_center, 
	image::{
		DynamicImage, 
		Rgba
	}
};

use crate::gameplay::{
	get_delta_time, 
	player::Axis
};

use macroquad::{
	math::{
		Rect, 
		Vec2
	}, 
	texture::{
		DrawTextureParams, 
		Texture2D
	}
};

use super::{
	textures::{
		downscale, 
		render_texture, 
		to_texture
	}, 
	access_image, 
	SCREEN_SCALE
};

pub trait TexturedObj {
	fn update_texture(&mut self);
}

#[derive(PartialEq)]
pub struct EntityTexture {
	pub sprite: Texture2D,

	pos: Vec2,

	pub moving: bool,
	anim_time: f32,

	dir_horizontal: Axis,
	dir_vertical: Axis
}

impl EntityTexture {
	pub fn new(sprite: Texture2D) -> Self {
		Self {
			sprite,

			pos: Vec2::new(0., 0.),

			moving: false,
			anim_time: 0.,

			dir_horizontal: Axis::None,
			dir_vertical: Axis::None
		}
	}

	/// Updates the texture with the provided texture data
	pub fn update(&mut self, pos: Vec2, dir_horizontal: Axis, dir_vertical: Axis, moving: bool) {
		self.pos = pos;
		self.moving = moving;

		if self.moving {
			self.dir_horizontal = dir_horizontal;
			self.dir_vertical = dir_vertical;
		}

		if self.moving && self.anim_time < 64. {
			self.anim_time += get_delta_time();
		} else {
			self.anim_time = 0.;
		}
	}

	/// Renders the texture with the current texture data
	pub async fn render(&self) {
		let size = self.sprite.height() / 5.;

		// There is definitely a far better way to do this
		// I apologize to whoever has to deal with this in the future 
		let y_pos = if self.dir_horizontal != Axis::None && self.dir_vertical != Axis::None {
			if self.dir_vertical == Axis::Positive {
				size       // Diagonal up
			} else {
				size * 3.  // Diagonal down
			}
		} else if self.dir_horizontal != Axis::None {
			size * 2.      // Left/right
		} else {
			if self.dir_vertical == Axis::Positive {
				0.         // Down
			} else {
				size * 4.  // Up
			}
		};

		let x_pos = match self.anim_time as isize / 16 {
			1 => size,
			3 => size * 2.,
			_ => 0.
		};

		render_texture(
			&self.sprite, 
			Vec2::new(
				self.pos.x + self.sprite.width(), 
				self.pos.y + self.sprite.height()
			), 
			Some(DrawTextureParams {
				source: Some(
					Rect::new(
						x_pos,
						y_pos,
						size,
						size
					)
				),
				flip_x: if self.dir_horizontal == Axis::Negative {
					true
				} else {
					false
				},
				dest_size: Some(Vec2::new(size * SCREEN_SCALE, size * SCREEN_SCALE)),
				..Default::default()
			})
		).await;
	}
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum AttackTextureType {
	Slash,
	Dash,

	Burst,

	ProjectilePlayer,
	ProjectileEnemy
}

#[derive(Clone)]
pub struct AttackTexture {
	pub sprite: DynamicImage,
	pub anim_time: f32,

	pos: Vec2,
	angle: f32,
	size: u32,

	texturetype: AttackTextureType
}

impl AttackTexture {
	/// Creates an attack texture with a "slash" sprite
	pub fn new(pos: Vec2, size: f32, angle: f32, texturetype: AttackTextureType) -> Self {
		let size = size as u32;
		Self {
			sprite: downscale(
				match texturetype {
					// Physical
					AttackTextureType::Slash => access_image("default:attacks/slash"),
					AttackTextureType::Dash => access_image("default:attacks/dash"),

					// Burst
					AttackTextureType::Burst => access_image("default:attacks/burst"),
					
					// Projectile
					AttackTextureType::ProjectilePlayer => access_image("default:attacks/projectile-player"),
					AttackTextureType::ProjectileEnemy => access_image("default:attacks/projectile-enemy"),
				},
				size,
			),
			anim_time: 9.,

			pos,
			angle,
			size,

			texturetype
		}
	}

	pub fn update(&mut self, pos: Vec2, angle: f32) {
		self.pos = pos;
		self.angle = angle;

		match self.texturetype {
			AttackTextureType::ProjectileEnemy | AttackTextureType::ProjectilePlayer => (),
			_ => self.anim_time -= get_delta_time()
		}
	}

	pub async fn render(&self) {
		let mut x_pos = match self.anim_time as isize / 3 {
			2 => 0,
			1 => self.size,
			_ => self.size * 2,
		};

		if self.texturetype == AttackTextureType::ProjectileEnemy
		|| self.texturetype == AttackTextureType::ProjectilePlayer {
			x_pos = 0
		}

		render_texture(
			&to_texture(
				DynamicImage::ImageRgba8(rotate_about_center(
					self.sprite.crop_imm(
						x_pos, 
						0, 
						self.size, 
						self.size
					).as_rgba8().unwrap(), 
					self.angle,
					imageproc::geometric_transformations::Interpolation::Nearest, 
					Rgba {
						0: [0, 0, 0, 0] // Clear
					}
				))
			),
			self.pos, 
			None
		).await;
	}
}
