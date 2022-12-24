struct CameraUniform {
	view_pos: vec4<f32>,
	view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) color: vec3<f32>,
}

struct InstanceInput {
	@location(5) model_matrix_0: vec4<f32>,
	@location(6) model_matrix_1: vec4<f32>,
	@location(7) model_matrix_2: vec4<f32>,
	@location(8) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
	@builtin(position) builtin_position: vec4<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) color: vec3<f32>,
	@location(3) position: vec3<f32>,
}

@vertex 
fn vs_main(
	in: VertexInput,
	instance: InstanceInput,
) -> VertexOutput {
	let model_matrix = mat4x4<f32>(
		instance.model_matrix_0,
		instance.model_matrix_1,
		instance.model_matrix_2,
		instance.model_matrix_3,
	);

	let model_space = model_matrix * vec4<f32>(in.position, 1.0);

	var out: VertexOutput;
	out.position = model_space.xyz;
	out.builtin_position = camera.view_proj * model_space;
	out.color = in.color;
	out.normal = in.normal;
	return out;
}

@fragment
fn fs_main(
	in: VertexOutput,
) -> @location(0) vec4<f32> {
	return vec4<f32>(in.color, 1.0);
}
