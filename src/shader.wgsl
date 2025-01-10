struct Params {
	max_iter: u32,
	scale: f32,
	size: vec2<f32>,
	center: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0), 
    );

    return vec4<f32>(positions[vertex_index], 0.0, 1.0);
}

@group(0) @binding(0) var<uniform> params: Params;

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
	let width = params.size.x;
	let height = params.size.y;

	let aspect_ratio = width / height;

    let normalized_x = position.x / width - 0.5;
    let normalized_y = position.y / height - 0.5;

    let x = normalized_x * params.scale * aspect_ratio + params.center.x;
    let y = normalized_y * params.scale + params.center.y;

	let escape_radius = f32(1u << 16u);

    var z_re = 0.0;
    var z_im = 0.0;
	var z_re2 = 0.0;
	var z_im2 = 0.0;

    var iter = 0u;

    while (iter < params.max_iter) {
		z_im = 2.0 * z_re * z_im + y;
		z_re = z_re2 - z_im2 + x;

		z_re2 = z_re * z_re;
		z_im2 = z_im * z_im;

		if (z_re2 + z_im2 > escape_radius) {
			break;
		}

		iter += 1u;
    }

	if (iter < params.max_iter) {
		let smooth_iter = f32(iter) + 1.0 - log2(log2(z_re2 + z_im2));
		let t = smooth_iter / f32(params.max_iter);

		return vec4<f32>(
			sin(t * 5.0),
			sin(t * 10.0),
			sin(t * 15.0),
			1.0
		);
	}

    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
