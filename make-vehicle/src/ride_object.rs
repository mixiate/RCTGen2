use crate::ride_desc;
use openrct2::objects::ride;

pub fn create_sprite_groups(ride_desc: &ride_desc::Ride, vehicle: &ride_desc::Vehicle) -> ride::SpriteGroups {
    use ride_desc::SpriteGroup;

    let restraint_animation = vehicle
        .flags
        .as_ref()
        .and_then(|x| x.contains(&ride_desc::VehicleFlag::RestraintAnimation).then_some(4));

    let slopes60_banked22 = if ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls) {
        if ride_desc.sprites.contains(&SpriteGroup::DiveLoops) {
            Some(8)
        } else {
            Some(4)
        }
    } else {
        None
    };

    ride::SpriteGroups {
        slope_flat: ride_desc.sprites.contains(&SpriteGroup::Flat).then_some(32),
        slopes12: ride_desc.sprites.contains(&SpriteGroup::GentleSlopes).then_some(4),
        slopes25: ride_desc.sprites.contains(&SpriteGroup::GentleSlopes).then_some(32),
        slopes42: ride_desc.sprites.contains(&SpriteGroup::SteepSlopes).then_some(8),
        slopes60: ride_desc.sprites.contains(&SpriteGroup::SteepSlopes).then_some(32),
        slopes75: ride_desc.sprites.contains(&SpriteGroup::VerticalSlopes).then_some(4),
        slopes90: ride_desc.sprites.contains(&SpriteGroup::VerticalSlopes).then_some(32),
        slopes_loop: ride_desc.sprites.contains(&SpriteGroup::VerticalSlopes).then_some(4),
        slope_inverted: ride_desc.sprites.contains(&SpriteGroup::VerticalSlopes).then_some(4),
        slopes8: ride_desc.sprites.contains(&SpriteGroup::Diagonals).then_some(4),
        slopes16: ride_desc.sprites.contains(&SpriteGroup::Diagonals).then_some(4),
        slopes50: ride_desc.sprites.contains(&SpriteGroup::Diagonals).then_some(4),
        flat_banked22: ride_desc.sprites.contains(&SpriteGroup::BankedTurns).then_some(8),
        flat_banked45: ride_desc.sprites.contains(&SpriteGroup::BankedTurns).then_some(32),
        flat_banked67: ride_desc.sprites.contains(&SpriteGroup::InlineTwists).then_some(4),
        flat_banked90: ride_desc.sprites.contains(&SpriteGroup::InlineTwists).then_some(4),
        inline_twists: ride_desc.sprites.contains(&SpriteGroup::InlineTwists).then_some(4),
        slopes12_banked22: ride_desc.sprites.contains(&SpriteGroup::SlopeBankTransition).then_some(32),
        slopes8_banked22: ride_desc.sprites.contains(&SpriteGroup::DiagonalBankTransition).then_some(4),
        slopes25_banked22: ride_desc.sprites.contains(&SpriteGroup::SlopedBankTransition).then_some(4),
        slopes8_banked45: ride_desc.sprites.contains(&SpriteGroup::DiagonalSlopedBankTransition).then_some(4),
        slopes16_banked22: ride_desc.sprites.contains(&SpriteGroup::DiagonalSlopedBankTransition).then_some(4),
        slopes16_banked45: ride_desc.sprites.contains(&SpriteGroup::DiagonalSlopedBankTransition).then_some(4),
        slopes25_banked45: ride_desc.sprites.contains(&SpriteGroup::BankedSlopedTurns).then_some(32),
        slopes12_banked45: ride_desc.sprites.contains(&SpriteGroup::BankedSlopeTransition).then_some(4),
        slopes25_banked67: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
        slopes25_banked90: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
        slopes25_inline_twists: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
        slopes42_banked22: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
        slopes42_banked45: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
        slopes42_banked67: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
        slopes42_banked90: ride_desc.sprites.contains(&SpriteGroup::ZeroGRolls).then_some(4),
        slopes60_banked22,
        slopes50_banked45: ride_desc.sprites.contains(&SpriteGroup::DiveLoops).then_some(8),
        slopes50_banked67: ride_desc.sprites.contains(&SpriteGroup::DiveLoops).then_some(8),
        slopes50_banked90: ride_desc.sprites.contains(&SpriteGroup::DiveLoops).then_some(8),
        corkscrews: ride_desc.sprites.contains(&SpriteGroup::Corkscrews).then_some(4),
        restraint_animation,
    }
}

fn create_car(ride_desc: &ride_desc::Ride, vehicle: &ride_desc::Vehicle) -> ride::Car {
    let rotation_frame_mask = if vehicle.model.is_empty() { 0 } else { 31 };

    let num_seats = vehicle.capacity.unwrap_or(0);
    let num_seat_rows = vehicle.riders.as_ref().map(|x| x.len() as i32).unwrap_or(0);

    let car_visual = vehicle.model.is_empty().then_some(1);

    let sprite_groups = if vehicle.model.is_empty() {
        ride::SpriteGroups {
            slope_flat: Some(1),
            ..ride::SpriteGroups::default()
        }
    } else {
        create_sprite_groups(ride_desc, vehicle)
    };

    let has_additional_colour1 =
        vehicle.flags.as_ref().is_some_and(|x| x.contains(&ride_desc::VehicleFlag::SecondaryRemap));
    let has_additional_colour2 =
        vehicle.flags.as_ref().is_some_and(|x| x.contains(&ride_desc::VehicleFlag::TertiaryRemap));
    let has_screaming_riders =
        vehicle.flags.as_ref().is_some_and(|x| x.contains(&ride_desc::VehicleFlag::RidersScream));

    let loading_positions = vehicle
        .riders
        .iter()
        .flatten()
        .flat_map(|rider| {
            let position = (32.0 * rider.position[2]).round() as i32;
            if num_seats > 1 {
                vec![position - 1, position + 1]
            } else {
                vec![position]
            }
        })
        .collect();

    ride::Car {
        rotation_frame_mask,
        spacing: (vehicle.spacing * 278912.0) as i32,
        mass: vehicle.mass,
        num_seats,
        num_seat_rows,
        friction_sound_id: ride_desc.running_sound as i32,
        sound_range: ride_desc.secondary_sound as i32,
        car_visual,
        draw_order: vehicle.draw_order,
        sprite_groups,
        has_additional_colour1,
        has_additional_colour2,
        has_screaming_riders,
        loading_positions,
    }
}

pub fn create_ride_object(
    ride_desc: &ride_desc::Ride,
    images: Vec<openrct2::objects::image::Image>,
) -> ride::RideObject {
    let head_cars: Vec<i32> =
        ride_desc.configuration.front.iter().chain(ride_desc.configuration.second.iter()).copied().collect();

    let no_collision_crashes =
        ride_desc.flags.as_ref().is_some_and(|x| x.contains(&ride_desc::Flag::NoCollisionCrashes));
    let rider_controls_speed =
        ride_desc.flags.as_ref().is_some_and(|x| x.contains(&ride_desc::Flag::RiderControlsSpeed));

    let cars = ride_desc.vehicles.iter().map(|vehicle| create_car(ride_desc, vehicle)).collect();

    let properties = ride::Properties {
        ride_type: ride_desc.ride_type,
        category: ride::Category::Rollercoaster,
        limit_air_time_bonus: ride_desc.limit_air_time_bonus,
        min_cars_per_train: ride_desc.min_cars_per_train,
        max_cars_per_train: ride_desc.max_cars_per_train,
        num_empty_cars: ride_desc.zero_cars,
        tab_car: ride_desc.preview_tab_car,
        default_car: ride_desc.configuration.default,
        head_cars: (!head_cars.is_empty()).then_some(head_cars),
        tail_cars: ride_desc.configuration.rear.map(|x| vec![x]),
        build_menu_priority: ride_desc.build_menu_priority,
        no_collision_crashes,
        rider_controls_speed,
        rating_multipliers: ride_desc.rating_multipliers,
        max_height: ride_desc.max_height,
        car_colours: ride_desc.default_colors.iter().map(|x| vec![*x]).collect(),
        cars,
    };

    let strings = ride::ObjectStrings {
        name: ride::ObjectString {
            en_gb: ride_desc.name.clone(),
        },
        description: ride::ObjectString {
            en_gb: ride_desc.description.clone(),
        },
        capacity: ride::ObjectString {
            en_gb: ride_desc.capacity.clone(),
        },
    };

    ride::RideObject {
        id: ride_desc.id.clone(),
        original_id: ride_desc.original_id.clone(),
        version: ride_desc.version.clone().unwrap_or("1.0".to_string()),
        authors: vec![ride_desc.author.clone()],
        object_type: ride::ObjectType::Ride,
        properties,
        strings,
        images,
    }
}

pub fn save_ride_object(ride_object: &ride::RideObject, path: &std::path::Path) -> anyhow::Result<()> {
    use anyhow::Context as _;
    use serde::Serialize as _;

    let json_formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut json_buffer = Vec::new();
    let mut json_serializer = serde_json::Serializer::with_formatter(&mut json_buffer, json_formatter);

    ride_object.serialize(&mut json_serializer).with_context(|| "Could not serialize object json")?;

    std::fs::write(path, json_buffer).with_context(|| format!("Could not write object file {}", path.display()))?;

    Ok(())
}
