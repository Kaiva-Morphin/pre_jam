//! This module contains an implementation for [tiled::ResourceReader]

use bevy::asset::LoadContext;
use std::{
    io::{Cursor, Error as IoError, ErrorKind, Read},
    path::Path,
    sync::Arc,
};

pub(crate) struct BytesResourceReader<'a, 'b> {
    bytes: Arc<[u8]>,
    context: &'a mut LoadContext<'b>,
}
impl<'a, 'b> BytesResourceReader<'a, 'b> {
    pub(crate) fn new(bytes: &'a [u8], context: &'a mut LoadContext<'b>) -> Self {
        Self {
            bytes: Arc::from(bytes),
            context,
        }
    }
}

impl<'a> tiled::ResourceReader for BytesResourceReader<'a, '_> {
    type Resource = Box<dyn Read + 'a>;
    type Error = IoError;

    fn read_from(&mut self, path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
        if let Some(extension) = path.extension() {
            if extension == "tsx" {
                if let Some(f) = path.file_name() {
                    if let Some(f) = f.to_str() {
                        match f {
                            // "tilepack.tsx" => return Ok(Box::new(Cursor::new(include_bytes!("../../../game/assets/tilemaps/test/tilepack.tsx")))),
                            "tilemap.tsx" => return Ok(Box::new(Cursor::new(include_bytes!("../../../game/assets/tilemaps/v1.0/tilemap.tsx")))),
                            "pad_test.tsx" => return Ok(Box::new(Cursor::new(include_bytes!("../../../game/assets/tilemaps/v1.0/pad_test.tsx")))),
                            "test.tsx" => return Ok(Box::new(Cursor::new(include_bytes!("../../../game/assets/tilemaps/v1.0/test.tsx")))),
                            _ => {panic!("PLEASE ADD THE TILESET \"{:?}\" INTO libs/bevy_ecs_tiled/src/reader.rs", path.to_str())}
                        }
                    }
                }
            }
        }
        Ok(Box::new(Cursor::new(self.bytes.clone())))
    }
}


