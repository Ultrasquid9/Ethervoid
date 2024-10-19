use macroquad::math::Vec2;

use super::movement::Entity;

pub enum AttackTypes {
	Physical(PhysicalOrBurst),
	Burst(PhysicalOrBurst),
	Projectile(ProjectileOrHitscan),
	Hitscan(ProjectileOrHitscan)
}

pub enum AttackAttributes {
	Explosive,
	Electric
}

pub struct PhysicalOrBurst {
	pos: Vec2,
	damage: u8,
	radius: u8
}

pub struct ProjectileOrHitscan {
	start_pos: Vec2,
	end_pos: Vec2,
	damage: u8,
	attributes: AttackAttributes
}

impl AttackTypes {
	pub fn new_physical(pos: Vec2, damage: u8, radius: u8) -> AttackTypes {
		return AttackTypes::Physical(PhysicalOrBurst {
			pos,
			damage,
			radius
		})
	}

	pub fn new_burst(pos: Vec2, damage: u8, radius: u8) -> AttackTypes {
		return AttackTypes::Burst(PhysicalOrBurst {
			pos,
			damage,
			radius
		})
	}

	pub fn new_projectile(start_pos: Vec2, end_pos: Vec2, damage: u8, attributes: AttackAttributes) -> AttackTypes {
		return AttackTypes::Projectile(ProjectileOrHitscan {
			start_pos,
			end_pos,
			damage,
			attributes
		})
	}

	pub fn new_hitscan(start_pos: Vec2, end_pos: Vec2, damage: u8, attributes: AttackAttributes) -> AttackTypes {
		return AttackTypes::Hitscan(ProjectileOrHitscan {
			start_pos,
			end_pos,
			damage,
			attributes
		})
	}

	pub fn damage(&self, entity: &Entity) {
		match self {
			Self::Physical(attributes) => todo!(),
			Self::Burst(attributes) => todo!(),
			Self::Projectile(attributes) => todo!(),
			Self::Hitscan(attributes) => todo!()
		}
	}
}
