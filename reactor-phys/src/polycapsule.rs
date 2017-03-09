use arc::Arc;
use circle::Circle;
use fx32::Fx32;
use fx32::Fx32Small;
use ray2::Ray2;
use raytraceresult::Ray2TraceResult;
use vec2::Vec2;
use wide_line::WideLine;

pub struct PolyCapsule {
	pub arcs: Vec <Arc>,
	pub lines: Vec <WideLine>,
}

impl PolyCapsule {
	pub fn new (points: &[Vec2 <Fx32>], radius: Fx32) -> PolyCapsule
	{
		let count = points.len ();
		// A single circle is not a capsule
		assert! (count >= 2);
		
		let lines = {
			let mut lines = vec! [];
			
			lines.push (WideLine::new (points [0], points [1], radius));
			
			for i in 1..count {
				lines.push (WideLine::new (points [i - 1], points [i], radius));
			}
			
			lines
		};
		
		let arcs = {
			let mut arcs = vec! [];
			let circles: Vec <Circle> = points.iter ().map (|p| Circle { center: *p, radius: radius }).collect ();
			
			arcs.push (Arc::new1 (&circles [0], points [1]));
			
			for i in 1..count - 1 {
				arcs.push (Arc::new2 (&circles [i], points [i - 1], points [i + 1]));
			}
			
			arcs.push (Arc::new1 (&circles [count - 1], points [count - 2]));
			
			arcs
		};
		
		PolyCapsule {
			arcs: arcs,
			lines: lines,
		}
	}
	
	pub fn collect (capsules: &[PolyCapsule]) -> PolyCapsule {
		PolyCapsule {
			arcs: {
				let mut arcs = vec! [];
				
				for capsule in capsules.iter () {
					arcs.extend (capsule.arcs.clone ());
				}
				
				arcs
			},
			lines: {
				let mut lines = vec! [];
				
				for capsule in capsules.iter () {
					lines.extend (capsule.lines.clone ());
				}
				
				lines
			},
		}
	}
	
	pub fn affine <F> (&self, f: F) -> PolyCapsule where F: Fn (Vec2 <Fx32>) -> Vec2 <Fx32>
	{
		PolyCapsule {
			arcs: self.arcs.iter ().map (|a| Arc { circle: Circle { center: f (a.circle.center), radius: a.circle.radius }, rejected_normals: a.rejected_normals }).collect (),
			lines: self.lines.iter ().map (|l| WideLine::new (f (l.start), f (l.end), l.radius)).collect (),
		}
	}
	
	pub fn translate (&self, offset: Vec2 <Fx32>) -> PolyCapsule {
		self.affine (|p| p + offset)
	}
}
