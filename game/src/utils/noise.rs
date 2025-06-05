use std::fs::File;
use std::io::Write;

pub fn save_texture(data: &[u8], size: u32, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(data).unwrap();
}

use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use noise::{NoiseFn, OpenSimplex, Perlin};
use bevy::render::render_resource::{AddressMode, Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::prelude::*;
use simdnoise::NoiseBuilder;



fn seamless_2d_noise(x: f64, y: f64, period: f64) -> f64 {
    let noise = OpenSimplex::new(123);
    
    let nx = (x / period) * std::f64::consts::TAU;
    let ny = (y / period) * std::f64::consts::TAU;

    let sx = nx.cos();
    let sy = nx.sin();
    let sz = ny.cos();
    let sw = ny.sin();

    noise.get([sx, sy, sz, sw]) // 4D
}


pub fn create_texture_3d(data: &[u8], size: u32) -> Image {
    let size = Extent3d {
        width: size,
        height: size,
        depth_or_array_layers: size,
    };
    
    Image {
        texture_descriptor: TextureDescriptor {
            label: Some("noise texture"),
            size,
            dimension: TextureDimension::D3,
            format: TextureFormat::R8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        sampler: ImageSampler::Descriptor(
            ImageSamplerDescriptor{
                mag_filter: ImageFilterMode::Linear,
                min_filter: ImageFilterMode::Linear,
                mipmap_filter: ImageFilterMode::Linear,
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                address_mode_w: ImageAddressMode::Repeat,
                ..Default::default()
            },
        ),
        data: Some(data.to_vec()),
        ..default()
    }
}
