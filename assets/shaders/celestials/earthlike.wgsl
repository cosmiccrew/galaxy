// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_pbr::utils  random1D, PI
#import bevy_sprite::mesh2d_view_bindings  globals
#import galaxy::shader_types CelestialSettings, Colour

@group(1) @binding(0)
var<uniform> settings: CelestialSettings;
@group(1) @binding(1)
var<uniform> land_colours: array<Colour, 4>;
@group(1) @binding(2)
var<uniform> river_colours: array<Colour, 2>;


const OCTAVES = 6; // 0 -> 20
const size: f32 = 4.6;
const river_cutoff: f32 = 0.368; // 0. -> 1.
const dither_size: f32 = 3.951; // 0. -> 10.
const light_border_1: f32 = 0.287; // 0. -> 1.
const light_border_2: f32 = 0.476; // 0. -> 1.
const should_dither: bool = true; // bool
const light_origin: vec2<f32> = vec2<f32>(0.5, 0.5); // 0. -> 1.

fn random2D(coord: vec2<f32>) -> f32 {
	// land has to be tiled (or the contintents on this planet have to be changing very fast)
	// tiling only works for integer values, thus the rounding
	// it would probably be better to only allow integer sizes
	// multiply by vec2(2,1) to simulate planet having another side
	let coord = coord % (vec2(2.0,1.0) * round(size));
	return fract(sin(dot(coord.xy ,vec2(12.9898,78.233))) * 15.5453 * settings.seed);
}

fn noise(coord: vec2<f32>) -> f32 {
	let i: vec2<f32> = floor(coord);
	let f: vec2<f32> = fract(coord);
		
	let a: f32 = random2D(i);
    let b: f32 = random2D(i + vec2(1.0, 0.0));
    let c: f32 = random2D(i + vec2(0.0, 1.0));
    let d: f32 = random2D(i + vec2(1.0, 1.0));

	let cubic: vec2<f32> = f * f * (3. - 2. * f);

	return mix(a, b, cubic.x) + (c - a) * cubic.y * (1. - cubic.x) + (d - b) * cubic.x * cubic.y;
}

fn rotate(coord: vec2<f32>, angle: f32) -> vec2<f32> {
	var coord = coord;
	coord -= 0.5;
	coord *= mat2x2<f32>(vec2<f32>(cos(angle), -sin(angle)), vec2<f32>(sin(angle), cos(angle)));
	return coord + 0.5;
}

fn fbm(coord: vec2<f32>) -> f32 {
    var coord = coord;
	var value: f32 = 0.;
	var scale: f32 = 0.5;

	for(var i = 0; i < OCTAVES; i++) {
		value += noise(coord) * scale;
		coord *= 2.;
		scale *= 0.5;
	}
	return value;
}

fn dither(uv1: vec2<f32>, uv2: vec2<f32>) -> bool {
	return (uv1.x+uv2.y) % (2. / settings.pixels) <= 1. / settings.pixels;
}

fn spherify(uv: vec2<f32>) -> vec2<f32> {
	let centered = uv * 2. - 1.;
	let z = sqrt(1. - dot(centered.xy, centered.xy));
	// let z = pow(1. - dot(centered.xy, centered.xy), 0.5);
	let sphere = centered / (z + 1.);
	
	return sphere * 0.5 + 0.5;
}

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {	

    var uv = floor(mesh.uv * settings.pixels) / settings.pixels;

    let dith = dither(uv, mesh.uv);

    let alpha = step(length(uv - vec2(0.5)), 0.49999);

    uv = spherify(uv);

    var d_light = distance(uv, light_origin);

	uv = rotate(uv, settings.rotation);

    //replace time with globals.time
    let base_fbm_uv = uv * size + vec2(globals.time * settings.time_speed, 0.);

    var fbm1: f32 = fbm(base_fbm_uv);
    var fbm2: f32 = fbm(base_fbm_uv - light_origin * fbm1);
    var fbm3: f32 = fbm(base_fbm_uv - light_origin * 1.5 * fbm1);
    var fbm4: f32 = fbm(base_fbm_uv - light_origin * 2. * fbm1);

    var river_fbm: f32 = fbm(base_fbm_uv + fbm1 * 6.);

	river_fbm = step(river_cutoff, river_fbm);
	
	// size of edge in which colors should be dithered
	let dither_border: f32 = (1.0 / settings.pixels) * dither_size;

    // lots of magic numbers here
	// you can mess with them, it changes the color distribution
	if (d_light < light_border_1) {
		fbm4 *= 0.9;
	}
	if (d_light > light_border_1) {
		fbm2 *= 1.05;
		fbm3 *= 1.05;
		fbm4 *= 1.05;
	} 
	if (d_light > light_border_2) {
		fbm2 *= 1.3;
		fbm3 *= 1.4;
		fbm4 *= 1.8;
		
		if (d_light < light_border_2 + dither_border) {
			if (dith || !should_dither) {
				fbm4 *= 0.5;
			}
		}
	}
	
	// increase contrast on d_light
	d_light = pow(d_light, 2.) * 0.4;
	var colour: Colour = land_colours[3];
	if (fbm4 + d_light < fbm1*1.5) {
		colour = land_colours[2];
	}
	if (fbm3 + d_light < fbm1 * 1.) {
		colour = land_colours[1];
	}
	if (fbm2 + d_light < fbm1) {
		colour = land_colours[0];
	}
	if (river_fbm < fbm1 * 0.5) {
		colour = river_colours[1];
		if (fbm4 + d_light < fbm1 * 1.5) {
			colour = river_colours[0];
		}
	}

	return vec4<f32>(colour.colour.rgb, colour.colour.a * alpha);
}
