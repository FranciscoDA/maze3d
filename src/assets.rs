use raylib::prelude::*;

pub struct AssetPack {
    pub tex_start: Texture2D,
    pub tex_smiley: Texture2D,
    pub tex_wall: Texture2D,
    pub tex_floor: Texture2D,
    pub tex_ceiling: Texture2D,
    pub tex_rat: Texture2D,
    pub tex_opengl: Texture2D,
    pub model_dodecahedron: Model,
    pub shader_diffuse: Shader,
}

impl AssetPack {
    pub fn init(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut result = Self {
            tex_start: rl.load_texture(&thread, "assets/3dmaze/start.png").unwrap(),
            tex_smiley: rl.load_texture(&thread, "assets/3dmaze/smiley.png").unwrap(),
            tex_wall: rl.load_texture(&thread, "assets/3dmaze/wall.png").unwrap(),
            tex_floor: rl.load_texture(&thread, "assets/3dmaze/floor.png").unwrap(),
            tex_ceiling: rl.load_texture(&thread, "assets/3dmaze/ceiling.png").unwrap(),
            tex_rat: rl.load_texture(&thread, "assets/3dmaze/rat.png").unwrap(),
            tex_opengl: rl.load_texture(&thread, "assets/3dmaze/opengl.png").unwrap(),
            model_dodecahedron: rl.load_model(&thread, "assets/dodecahedron.obj").unwrap(),
            shader_diffuse: rl
                .load_shader(&thread, Some("assets/diffuse.vs"), Some("assets/diffuse.fs"))
                .unwrap(),
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

        result.model_dodecahedron.materials_mut()[0].shader = *result.shader_diffuse;

        return result;
    }
}
