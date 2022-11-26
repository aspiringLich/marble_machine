use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait AtlasDictionary
where
    Self: Sized + Copy + Clone + IntoEnumIterator,
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
    /// let (texture_atlas, index, scale) = atlas::get_sprite(atlas::item1);
    /// ```
    fn info(self) -> (Handle<TextureAtlas>, usize, Vec3) {
        (
            Self::get(),
            self.index(),
            (self.rect().size() * GRID_SIZE).extend(1.0),
        )
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
            atlas.add_texture(Rect::from_corners(
                rect.min * GRID_SIZE,
                rect.max * GRID_SIZE,
            ));
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

macro rect($a:expr,$b:expr,$c:expr,$d:expr) {
    Rect::new($a as f32, $b as f32, $c as f32, $d as f32)
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, EnumIter)]
pub enum basic {
    marble_small,
    marble,
    marble_output,
    marble_input,
    body_small,
    body,
}

impl AtlasDictionary for basic {
    fn rect(self) -> Rect {
        use basic::*;

        #[rustfmt::skip]
        match self {
            marble_small  => rect!(0, 0, 1, 1),
            marble        => rect!(1, 0, 2, 1),
            marble_output => rect!(0, 1, 1, 3),
            marble_input  => rect!(0, 3, 1, 5),
            body_small    => rect!(3, 0, 5, 2),
            body          => rect!(5, 0, 8, 3),
        }
    }

    default_impl_atlas_dictionary!(basic, 0);
}
