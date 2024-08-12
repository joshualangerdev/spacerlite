use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ImageHandles>();
    app.init_resource::<ImageHandles>();
}

#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct ImageHandles(HashMap<String, Handle<Image>>);

impl ImageHandles {
    pub const TEST_SHIP: &'static str = "ships/bgbattleship.png";
}

impl FromWorld for ImageHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let pixel_art_settings = |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        };

        let pixel_art_paths = [Self::TEST_SHIP];
        let map = pixel_art_paths
            .into_iter()
            .map(|path| {
                (
                    path.to_string(),
                    asset_server.load_with_settings(path, pixel_art_settings),
                )
            })
            .collect();

        Self(map)
    }
}
