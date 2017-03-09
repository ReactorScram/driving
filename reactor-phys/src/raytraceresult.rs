use fx32::Fx32;
use fx32::Fx32Small;
use vec2::Vec2;

#[derive (Clone, Copy)]
pub enum Ray2TraceResult {
	Hit (Fx32Small, Vec2 <Fx32>, Vec2 <Fx32Small>),
	Pop (Vec2 <Fx32>, Vec2 <Fx32Small>),
	Miss,
}

impl Ray2TraceResult {
	pub fn fold (a: Ray2TraceResult, b: Ray2TraceResult) -> Ray2TraceResult {
		match a {
			Ray2TraceResult::Miss => {
				return b;
			},
			Ray2TraceResult::Hit (a_t, ..) => {
				match b {
					Ray2TraceResult::Miss => {
						return a;
					},
					Ray2TraceResult::Hit (b_t, ..) => {
						if a_t.x < b_t.x {
							return a;
						}
						else {
							return b;
						}
					},
					Ray2TraceResult::Pop (..) => {
						return b;
					}
				}
			},
			Ray2TraceResult::Pop (..) => {
				return a;
			},
		}
	}
}

