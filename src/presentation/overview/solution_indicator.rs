use bevy::prelude::*;
use bevy_escape_core::{Effect, apply_effects};

use crate::domain::item::{GiveItem, Inventory, ItemId};
use crate::domain::solved::{Solution1, Solved};
use crate::layout::HotspotLayout;

#[derive(Component)]
pub struct SolutionIndicator;

pub fn draw_solution_indicator(commands: &mut Commands, layout: &HotspotLayout) {
    commands
        .spawn((
            SolutionIndicator,
            Sprite {
                color: Color::NONE,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_right_x, layout.top_row_y, 0.0),
            Visibility::Hidden,
            Pickable::default(),
        ))
        .observe(on_solution_indicator_click);
}

fn on_solution_indicator_click(
    _click: On<Pointer<Click>>,
    solved: Res<Solved>,
    inventory: Res<Inventory>,
    mut commands: Commands,
) {
    let item = match solved.quiz1 {
        Solution1::Answer3 if !inventory.has(&ItemId::Key1) => Some(ItemId::Key1),
        Solution1::Answer12 if !inventory.has(&ItemId::Key2) => Some(ItemId::Key2),
        _ => None,
    };

    let Some(item) = item else {
        return;
    };

    commands.queue(move |world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(GiveItem { item })];
        apply_effects(effects, world);
    });
}

pub fn sync_solution_indicator(
    solved: Res<Solved>,
    mut indicators: Query<(&mut Sprite, &mut Visibility), With<SolutionIndicator>>,
) {
    if !solved.is_changed() {
        return;
    }

    let Ok((mut sprite, mut visibility)) = indicators.single_mut() else {
        return;
    };

    match solved.quiz1 {
        Solution1::Unsolved => {
            *visibility = Visibility::Hidden;
        }
        Solution1::Answer3 => {
            sprite.color = Color::srgb(0.2, 0.8, 0.2);
            *visibility = Visibility::Visible;
        }
        Solution1::Answer12 => {
            sprite.color = Color::srgb(0.6, 0.2, 0.8);
            *visibility = Visibility::Visible;
        }
    }
}
