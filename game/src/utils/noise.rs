use std::fs::File;
use std::io::Write;

pub fn save_texture(data: &[u8], size: u32, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(data).unwrap();
}

use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::prelude::*;


pub fn get_noise_3d() -> Image {
    let size = 128;
    let mut data = include_bytes!("../../assets/shaders/noise.bin");
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
