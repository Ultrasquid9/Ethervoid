use macroquad::math::Vec2;
use raylite::{cast_wide, Barrier, Ray};
use rhai::{CustomType, TypeBuilder};

use super::{cores::map::Map, draw::texturedobj::{AttackTexture, AttackTextureType, TexturedObj}, enemy::Enemy, entity::{Entity, MovableObj}, get_delta_time, get_mouse_pos, player::Player, vec2_to_tuple};

#[derive(Clone)]
pub struct Attack {
	pub size: f32,
	pub pos: Vec2,
	target: Vec2,

	pub owner: Owner,
	pub is_parried: bool,

	attack_type: AttackType,
	damage: isize,
	lifetime: f32,

	pub texture: AttackTexture
}

#[derive(Clone, PartialEq)]
pub enum Owner {
	Player,
	Enemy
}

#[derive(Clone, PartialEq)]
pub enum AttackType {
	Physical,
	Burst,
	Projectile(ProjectileOrHitscan),
	Hitscan(ProjectileOrHitscan)
}

#[derive(Clone, PartialEq)]
pub struct ProjectileOrHitscan {}

impl Attack {
	pub fn new_physical(pos: Vec2, target: Vec2, damage: isize, size: f32, owner: Owner, texturetype: AttackTextureType) -> Attack {
		return Attack {
			size,
			pos,
			target,

			owner,
			is_parried: false,

			attack_type: AttackType::Physical,
			damage,
			lifetime: 2.,

			// After angle_between led to wierd-ass bugs, I added atan2 and it worked.
			// I have as such concluded that atan2 is magical, and fixes every problem. 
			texture: AttackTexture::new(
				pos, 
				size, 
				(target.y).atan2(target.x), 
				texturetype
			)
		}
	}

	pub fn new_burst(pos: Vec2, damage: isize, size: f32, owner: Owner, texturetype: AttackTextureType) -> Attack {
		return Attack {
			size,
			pos,
			target: pos,

			owner,
			is_parried: false,

			attack_type: AttackType::Burst,
			damage,
			lifetime: 12.,

			texture: AttackTexture::new(
				pos, 
				size, 
				0., 
				texturetype
			)
		}
	}

	pub fn new_projectile(pos: Vec2, target: Vec2, damage:isize, owner: Owner, texturetype: AttackTextureType) -> Attack {
		return Attack {
			size: 10.,
			pos,
			target: ((target - pos) * 999.) + pos,

			owner,
			is_parried: false,

			attack_type: AttackType::Projectile( ProjectileOrHitscan {}),
			damage,
			lifetime: 1.,

			texture: AttackTexture::new(
				pos, 
				16., 
				0., 
				texturetype
			)
		}
	}

	pub fn new_hitscan(pos: Vec2, target: Vec2, damage: isize, owner: Owner) -> Attack {
		return Attack {
			size: 10.,
			pos,
			target,

			owner,
			is_parried: false,

			attack_type: AttackType::Hitscan( ProjectileOrHitscan {}),
			damage,
			lifetime: 8.,

			texture: AttackTexture::new(
				pos, 
				16., 
				0., 
				super::draw::texturedobj::AttackTextureType::Slash
			)
		}
	}

	/// Checks if the attack is a hitscan attack
	pub fn is_hitscan(&self) -> bool {
		match self.attack_type {
			AttackType::Hitscan(_) => true,
			_ => false
		}
	}

	/// Gets the target of the attack
	pub fn get_target(&self) -> Vec2 {
		self.target
	}

	/// Checks if the attack should be removed
	pub fn should_rm(&self) -> bool {
		match self.attack_type {
			AttackType::Physical | AttackType::Burst => {
				if self.texture.anim_time <= 0. {
					return true;
				}
			},
			AttackType::Projectile(_) | AttackType::Hitscan(_) => {
				if self.lifetime <= 0. {
					return true;
				}
			}
		}

		return false;
	}

	// The following code is for updating attacks
	// Be warned: expect horrors beyond human comprehension

	/// Updates the attack based upon its type
	pub fn update(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map: &Map) {
		self.update_texture();

		match &self.attack_type {
			AttackType::Physical => {
				if self.lifetime > 0. {
					self.attack_physical(enemies, player);
				}
			} 
			AttackType::Burst => {
				if self.lifetime > 0. {
					self.attack_burst(enemies, player);
				}
			}
			AttackType::Projectile(_) => self.attack_projectile(enemies, player, map),
			AttackType::Hitscan(_) => self.attack_hitscan(enemies, player),
		}
	}

	/// Updates the provided Physical attack
	fn attack_physical(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player) {
		match self.owner {
			Owner::Player => for i in enemies {
				if i.stats.is_touching(self) {
					i.stats.try_damage(self.damage);
				}
			}
			Owner::Enemy => if player.stats.is_touching(self) {
				player.stats.try_damage(self.damage);
			}
		}

		self.lifetime -= get_delta_time();
	}

	/// Updates the provided Burst attack
	fn attack_burst(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player) {
		// Returns the attack but with double the size
		let double_size = || {
			let mut to_return = self.clone();
			to_return.size *= 2.;

			return to_return;
		};

		match self.owner {
			Owner::Player => for i in enemies {
				if i.stats.is_touching(&mut double_size()) {
					i.stats.try_damage(self.damage * (i.stats.get_pos().distance(self.pos) / (self.size * 2.)) as isize);
				}
			}
			Owner::Enemy => if player.stats.is_touching(&mut double_size()) {
				player.stats.try_damage(self.damage * (player.stats.get_pos().distance(self.pos) / (self.size * 2.)) as isize);
			}
		}

		self.lifetime -= get_delta_time();
	}

	/// Updates the provided Projectile attack
	fn attack_projectile(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map: &Map) {
		match self.owner {
			Owner::Player => for i in enemies {
				if self.is_touching(&i.stats) {
					i.stats.try_damage(self.damage);
					self.lifetime = 0.;
					return;
				}
			}
			Owner::Enemy => if self.is_touching(&player.stats) {
				player.stats.try_damage(self.damage);
				self.lifetime = 0.;
				return;
			}
		}

		let new_pos = self.pos.move_towards(self.target, 5.0);
		let new_pos = ((new_pos - self.get_pos()) * get_delta_time()) + self.get_pos();
		self.try_move(new_pos, map);

		if self.pos != new_pos || self.pos.round() == self.target.round() {
			self.lifetime = 0.;
		}
	}

	/// Updates the provided Hitscan attack
	fn attack_hitscan(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player) {
		// Damages the provided entity with a raycast
		let damage_with_raycast = |entity: &mut Entity| {
			match cast_wide(
				&Ray{
					position: vec2_to_tuple(&self.pos), 
					end_position: vec2_to_tuple(&self.target)
				}, 
				&entity_to_barriers(entity)
			) {
				Ok(_) => entity.try_damage(self.damage),
				_ => ()
			}
		};
		
		match self.owner {
			Owner::Player => enemies
				.iter_mut()
				.for_each(|i| damage_with_raycast(&mut i.stats)),
			Owner::Enemy => damage_with_raycast(&mut player.stats),
		}

		self.lifetime -= get_delta_time();
	}
}

// Allows Attacks to be created by scripts
impl CustomType for Attack {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("attack");
	}
}

// Allows Attacks to be moved
impl MovableObj for Attack {
	fn get_size(&self) -> &f32 {
		&self.size
	}

	fn get_pos(&self) -> Vec2 {
		return self.pos
	}

	fn edit_pos(&mut self) -> &mut Vec2 {
		&mut self.pos
	}
}

impl TexturedObj for Attack {
	fn update_texture(&mut self) {
		self.texture.update(
			self.pos, 
			match self.attack_type {
				AttackType::Physical => {
					(self.target.y).atan2(self.target.x)
				},
				_ => {
					0.
				}
			}
		);
	}
}

/// Attempts to parry attacks 
pub fn try_parry(attacks: &mut Vec<Attack>) {
	// i is the index of the attack that is trying to parry other attacks
	for i in (0..attacks.len()).rev() {
		// Checking if i is not physical or has been parried, and continuing the loop if either is true
		if attacks[i].attack_type != AttackType::Physical
		|| attacks[i].is_parried {
			continue;
		}

		// Looping through all the other attacks
		for j in (0..attacks.len()).rev() {
			if i == j {
				continue;
			}

			// Coming up next: more nesting than the average bird

			// Checking if j is touching i and if j has not already been parried
			if attacks[j].is_touching(&attacks[i])
			&& !attacks[j].is_parried {
				// Checking the attack type of j
				match &attacks[j].attack_type {

					// Physical attacks
					AttackType::Physical => {
						if attacks[j].owner != attacks[i].owner {
							attacks[j].owner = attacks[i].owner.clone();

							attacks[j].damage += attacks[i].damage;
							
							attacks[i].lifetime += get_delta_time();
							attacks[j].lifetime += get_delta_time();

							attacks[i].is_parried = true;
							attacks[j].is_parried = true;
						}
					}

					// Projectile attacks
					AttackType::Projectile(_attributes) => {
						if attacks[j].owner != attacks[i].owner {
							attacks[j].owner = attacks[i].owner.clone();
						}

						attacks[j].attack_type = AttackType::Hitscan(ProjectileOrHitscan {});
						attacks[j].damage += attacks[i].damage;

						attacks[j].target = match attacks[j].owner {
							Owner::Player => get_mouse_pos() * 999.,
							Owner::Enemy => Vec2::new(0., 0.) // TODO - Get target of the enemy who owns this attack 
						};

						// Since hitscan attacks cannot be parried, the is_parried bool is unneccessary
					}

					// Burst and Hitscan attacks cannot be parried
					_ => ()
				}
			}
		}
	}
}

/// Converts the provided entity into two barriers, a horizontal and a vertical one 
fn entity_to_barriers(entity: &Entity) -> Vec<Barrier> {
	vec![
		Barrier {
			positions: (
				(entity.x() + entity.size, entity.y()),
				(entity.x() - entity.size, entity.y())
			)
		},
		Barrier {
			positions: (
				(entity.x(), entity.y() + entity.size),
				(entity.x(), entity.y() - entity.size)
			)
		}
	]
}
