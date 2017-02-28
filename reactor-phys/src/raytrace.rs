use circle::Circle;
use fx32::Fx32;
use ray2::Ray2;

pub enum RayTraceResult {
	Hit (Fx32),
	Miss,
}

pub fn ray_trace_circle (ray: &Ray2, circle: &Circle) -> RayTraceResult {
	RayTraceResult::Miss
}
