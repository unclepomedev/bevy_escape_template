use bevy::prelude::*;
use bevy_escape_core::{Effect, apply_effects};

use crate::domain::hotspot::Hotspot5;
use crate::domain::item::{GiveItem, ItemId};
use crate::domain::progress::Progress;
use crate::layout::HotspotLayout;

pub fn draw_hotspot5(commands: &mut Commands, layout: &HotspotLayout) {
    let color = Color::srgb(0.9, 0.7, 0.4);

    commands
        .spawn((
            Hotspot5,
            Sprite {
                color,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_center_x, layout.middle_row_y, 0.0),
            Visibility::Hidden,
            Pickable::default(),
        ))
        .observe(on_hotspot5_click);
}

fn on_hotspot5_click(_click: On<Pointer<Click>>, mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(GiveItem { item: ItemId::Key3 })];
        apply_effects(effects, world);
    });
}

pub fn sync_hotspot5_visibility(
    progress: Res<Progress>,
    mut indicators: Query<&mut Visibility, With<Hotspot5>>,
) {
    if !progress.is_changed() {
        return;
    }

    let Ok(mut visibility) = indicators.single_mut() else {
        return;
    };

    *visibility = if progress.rect5_unlocked {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}
