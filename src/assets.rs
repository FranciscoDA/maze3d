use raylib::prelude::*;

pub struct AssetPack {
    pub tex_start: Texture2D,
    pub tex_smiley: Texture2D,
    pub tex_wall: Texture2D,
    pub tex_floor: Texture2D,
    pub tex_ceiling: Texture2D,
    pub tex_rat: Texture2D,
    pub tex_opengl: Texture2D,
}

impl AssetPack {
    pub fn init(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let result = Self {
            tex_start: rl.load_texture(&thread, "res/start.png").unwrap(),
            tex_smiley: rl.load_texture(&thread, "res/smiley.png").unwrap(),
            tex_wall: rl.load_texture(&thread, "res/wall.png").unwrap(),
            tex_floor: rl.load_texture(&thread, "res/floor.png").unwrap(),
            tex_ceiling: rl.load_texture(&thread, "res/ceiling.png").unwrap(),
            tex_rat: rl.load_texture(&thread, "res/rat.png").unwrap(),
            tex_opengl: rl.load_texture(&thread, "res/opengl.png").unwrap(),
        };
        result
            .tex_wall
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
        result
            .tex_floor
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
        result
            .tex_ceiling
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
        return result;
    }
}
