use bevy::{ecs::entity::Entities, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_web_asset::WebAssetPlugin;
use cc::{get_creature_at_spacetime, DateTime, H3Cell, Spacetime, TimeZone, Utc};
use chrono::Duration;
use derive_deref::Deref;
use geo_types::Coordinate;
use googleprojection::{from_ll_to_subpixel, from_pixel_to_ll, Mercator};
use h3ron::ToCoordinate;
use slippy_map_tiles::{lat_lon_to_tile, merc_location_to_tile_coords, BBox, LatLon, Tile};

fn main() {
    App::new()
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(WebAssetPlugin)
        })
        .add_startup_system(setup)
        .add_plugin(PanCamPlugin::default())
        .insert_resource(Location((38.876860, -77.154240).into()))
        .insert_resource(LastCreatureUpdate(Utc.timestamp_nanos(0)))
        .add_system(spawn_and_despawn_creatures)
        .run();
}
pub struct Location(Coordinate<f64>);
const ZOOM: usize = 18;
fn setup(mut commands: Commands, location: Res<Location>) {
    let mut camera = commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    camera.insert(PanCam::default());
    let pix_loc = from_ll_to_subpixel(&location.0, ZOOM).unwrap();
    dbg!(&pix_loc);
    let cam_transform = Transform::from_xyz(pix_loc.x as f32, pix_loc.y as f32, 1.0);
    camera.insert(cam_transform);
}

pub struct LastCreatureUpdate(DateTime<Utc>);

fn spawn_and_despawn_creatures(
    mut lcu: ResMut<LastCreatureUpdate>,
    location: Res<Location>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &Spacetime)>,
) {
    let center_space = H3Cell::from_coordinate_unchecked(&location.0, 15);
    let nearby_spaces = center_space.k_ring(20);
    let now = Utc::now();
    let time_since_lcu = now - lcu.0;
    dbg!(&time_since_lcu);
    if time_since_lcu.num_minutes() >= 1 {
        //add creatures
        for delta_min in 1..=std::cmp::min(time_since_lcu.num_minutes(), 10) {
            nearby_spaces.iter().for_each(|space| {
                let time = now + Duration::minutes(delta_min);
                let creature = get_creature_at_spacetime(&Spacetime::new(space, time));
                if let Some(name) = creature {
                    let pixel = from_ll_to_subpixel(&space.to_coordinate(), ZOOM).unwrap();
                    let transform = Transform::from_xyz(pixel.x as f32, pixel.y as f32, 1.0);
                    commands.spawn_bundle(SpriteBundle {
                        texture: asset_server.load(name),
                        transform: transform,
                        ..Default::default()
                    });
                    // println!("added creature at {:?}", Spacetime::new(space, time));
                }
            });
        }
        //remove creatures
        for (e, st) in query.iter() {
            if (now - st.time).num_minutes() > 10 {
                commands.entity(e).despawn();
                // println!("removed creature at {:?}", st);
            }
        }
        lcu.0 = now;
    }
}
