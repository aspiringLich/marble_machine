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

macro rect {
    ($a:expr,$b:expr,$c:expr,$d:expr) => {
        Rect::new(
            $a as f32 * GRID_SIZE,
            $b as f32 * GRID_SIZE,
            $c as f32 * GRID_SIZE,
            $d as f32 * GRID_SIZE,
        )
    },
    ($a:expr,$b:expr,$c:expr,$d:expr,$pad:expr) => {
        Rect::new(
            ($a as f32 * GRID_SIZE) + $pad as f32,
            ($b as f32 * GRID_SIZE) + $pad as f32,
            ($c as f32 * GRID_SIZE) - $pad as f32,
            ($d as f32 * GRID_SIZE) - $pad as f32,
        )
    },
}

macro raw_rect($x:expr, $y:expr, $w:expr, $h:expr) {
    Rect::new(
        $x as f32,
        $y as f32,
        $x as f32 + $w as f32,
        $y as f32 + $h as f32,
    )
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
}

impl AtlasDictionary for basic {
    fn rect(self) -> Rect {
        use basic::*;

        #[rustfmt::skip]
        match self {
            corner        => rect!(0, 0, 1, 1),
            marble_small  => rect!(1, 0, 2, 1, 2),
            marble        => rect!(2, 0, 3, 1, 1),
            marble_output => rect!(0, 3, 1, 5),
            marble_input  => rect!(0, 1, 1, 3),
            body_small    => rect!(3, 0, 5, 2, 1),
            body          => rect!(5, 0, 8, 3, 1),
            target        => raw_rect!(8, 8, 7, 7),
            tracer_tick   => rect!(2, 1, 3, 2, 3),
        }
    }

    default_impl_atlas_dictionary!(basic, 0);
}
