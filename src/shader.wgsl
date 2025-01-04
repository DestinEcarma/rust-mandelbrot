struct Params {
	max_iter: u32,
	width: u32,
	height: u32,
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
    let normalized_x = position.x / f32(params.width);
    let normalized_y = position.y / f32(params.height);

	let aspect_ratio = f32(params.width) / f32(params.height);

    let x = (normalized_x - 0.5) * 3.0 * aspect_ratio;
    let y = (normalized_y - 0.5) * 3.0;

    var z_re = 0.0;
    var z_im = 0.0;

    var iter = 0u;

    while (iter < params.max_iter) {
		let z_re2 = z_re * z_re;
		let z_im2 = z_im * z_im;

		if (z_re2 + z_im2 > 4.0) {
			break;
		}

		z_im = 2.0 * z_re * z_im + y;
		z_re = z_re2 - z_im2 + x;

		iter += 1u;
    }

	if (iter < params.max_iter) {
		let t = f32(iter) / f32(params.max_iter);

		return vec4<f32>(
			sin(t * 5.0),
			sin(t * 10.0),
			sin(t * 15.0),
			1.0
		);
	}

    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
