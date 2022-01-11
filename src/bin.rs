use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_web_asset::WebAssetPlugin;
use cc::{get_creature_at_spacetime, H3Cell, Spacetime, TimeZone, Utc};
use geo_types::Coordinate;
use googleprojection::{from_ll_to_pixel, from_ll_to_subpixel, from_pixel_to_ll, Mercator};
use h3ron::ToCoordinate;
use slippy_map_tiles::{lat_lon_to_tile, BBox, LatLon, Tile};

fn main() {
    App::new()
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(WebAssetPlugin)
        })
        .add_startup_system(setup)
        .add_plugin(PanCamPlugin::default())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, wnds: Res<Windows>) {
    let zoom = 18;
    let home = (38.876860, -77.154240);
    let mut camera = commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    camera.insert(PanCam::default());
    let center_space = H3Cell::from_coordinate_unchecked(&home.into(), 15);
    let (x, y) = from_ll_to_subpixel(&home, zoom).unwrap();
    dbg!((&x, &y));
    let cam_transform = Transform::from_xyz(x as f32, y as f32, 1.0);
    camera.insert(cam_transform);
    let nearby_spaces = center_space.k_ring(20);
    nearby_spaces.iter().for_each(|space| {
        let time = Utc.ymd(2002, 12, 28).and_hms(0, 1, 0);
        let creature = get_creature_at_spacetime(&Spacetime::new(space, time));
        if let Some(name) = creature {
            let pixel = from_ll_to_subpixel(&space.to_coordinate().x_y(), zoom).unwrap();
            let transform = Transform::from_xyz(pixel.0 as f32, pixel.1 as f32, 1.0);
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load(name),
                transform: transform,
                ..Default::default()
            });
        }
    });
    let win = wnds.get_primary().unwrap();
    let half_width = win.width() as f64;
    let half_height = win.height() as f64;
    let tl = from_pixel_to_ll(&(x + half_width, y + half_height), zoom).unwrap();
    let tl = LatLon::new(tl.0 as f32, tl.1 as f32).unwrap();
    let br = from_pixel_to_ll(&(x - half_width, y - half_height), zoom).unwrap();
    let br = LatLon::new(br.0 as f32, br.1 as f32).unwrap();
    let bbox = BBox::new(tl.lat(), tl.lon(), br.lat(), br.lon()).unwrap();
    dbg!(&bbox);
    bbox.tiles_for_zoom(zoom as u8).for_each(|tile: Tile| {
        let ll = tile.center_point();
        let ll = (ll.lat() as f64, ll.lon() as f64);
        let pixel = from_ll_to_subpixel(&ll, zoom).unwrap();
        let transform = Transform::from_xyz(pixel.0 as f32, pixel.1 as f32, 1.0);
        dbg!((y as f32) - transform.translation.y);
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("tile.png"),
            transform,
            ..Default::default()
        });
    });
}
