use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::misc::builder_fn;

/// implement on an enum to have it be a valid key to an atlas
pub trait AtlasDictionary
where
    Self: Sized + Copy + Clone,
{
    /// the rect each individual item represents
    fn rect(self) -> Rect;
    /// the path to the atlas this enum is referring to
    fn path() -> String;
    /// the id of this atlas (which order we init it)
    fn atlas_n() -> usize;
    /// the id of this item in the atlas
    fn index(self) -> usize;

    /// get the texture handle for this atlas
    fn get() -> Handle<TextureAtlas> {
        unsafe { ATLAS_HANDLES[Self::atlas_n()].clone() }
    }

    /// get the information needed to create a sprite
    /// will return the handle, the id, and the size of the specific sprite
    ///
    /// ```
    /// let (texture_atlas, index) = atlas::get_sprite(atlas::item1);
    /// ```
    fn info(self) -> (Handle<TextureAtlas>, usize) {
        (Self::get(), self.index())
    }

    /// get width of self.rect()
    fn width(self) -> f32 {
        self.rect().width()
    }

    /// get height of self.rect()
    fn height(self) -> f32 {
        self.rect().height()
    }
}

#[derive(Deref, DerefMut, Default)]
pub struct SpriteSheetBuilder(SpriteSheetBundle);

impl SpriteSheetBuilder {
    pub fn new() -> Self {
        default()
    }

    builder_fn!(transform, Transform);
    builder_fn!(visibility, Visibility);
    builder_fn!(color, Color, sprite.color);
    builder_fn!(flip_x, bool, sprite.flip_x);
    builder_fn!(flip_y, bool, sprite.flip_y);
    builder_fn!(custom_size, Vec2, {
        sprite.custom_size = Some(custom_size)
    });
    builder_fn!(anchor, bevy::sprite::Anchor, sprite.anchor);

    pub fn build<T: AtlasDictionary>(mut self, input: T) -> SpriteSheetBundle {
        let (atlas, index) = input.info();
        self.texture_atlas = atlas;
        self.sprite.index = index;
        self.0
    }
}

const GRID_SIZE: f32 = 8.0;

/// kinda bad but its the easiest way i could think of
/// stores every handle for all the atlases we init
static mut ATLAS_HANDLES: Vec<Handle<TextureAtlas>> = vec![];

pub fn init_texture_atlas(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    macro init_atlas($target:ty, $dimensions:expr) {{
        let mut atlas =
            TextureAtlas::new_empty(asset_server.load(<$target>::path()), $dimensions.into());
        <$target>::iter().for_each(|variant| {
            let rect = variant.rect();
            atlas.add_texture(rect);
        });
        let handle = texture_atlases.add(atlas);
        unsafe { ATLAS_HANDLES.push(handle) };
    }}

    init_atlas!(basic, [64.0, 64.0]);
}

macro default_impl_atlas_dictionary($t:ty, $n:expr) {
    fn path() -> String {
        stringify!($t).to_string() + ".png"
    }
    fn atlas_n() -> usize {
        $n
    }
    fn index(self) -> usize {
        self as usize
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, EnumIter)]
pub enum basic {
    corner,
    marble_small,
    marble,
    marble_output,
    marble_input,
    body_small,
    body,
    target,
    tracer_tick,
    indicator,
}

impl AtlasDictionary for basic {
    fn rect(self) -> Rect {
        use basic::*;

        #[rustfmt::skip]
        let args = match self {
            corner        => (7, 3, 8, 8),
            marble_small  => (0, 0, 4, 4),
            marble        => (1, 0, 6, 6),
            marble_output => (0, 2, 8, 10),
            marble_input  => (0, 4, 8, 12),
            body_small    => (3, 0, 14, 14),
            body          => (5, 0, 22, 22),
            target        => (1, 1, 7, 7),
            tracer_tick   => (2, 1, 2, 2),
            indicator     => (0, 1, 3, 3),
        };

        Rect::new(
            args.0 as f32 * GRID_SIZE,
            args.1 as f32 * GRID_SIZE,
            args.0 as f32 * GRID_SIZE + args.2 as f32,
            args.1 as f32 * GRID_SIZE + args.3 as f32,
        )
    }

    default_impl_atlas_dictionary!(basic, 0);
}
