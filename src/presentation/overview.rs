use crate::domain::clear::{ClearLabel, SetGameClear};
use crate::domain::hotspot::{Hotspot1, Hotspot2, Hotspot4, Hotspot5};
use crate::domain::item::{GiveItem, Inventory, ItemId, remove_item_at};
use crate::domain::progress::{Progress, UnlockRect5};
use crate::domain::selection::SelectedSlot;
use crate::domain::solved::{Solution1, Solved};
use crate::layout::{HotspotLayout, calculate_hotspot_layout};
use crate::state::ZoomState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_escape_core::{Effect, apply_effects};

#[derive(Component)]
pub struct SolutionIndicator;

pub fn draw_hotspots(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows
        .single()
        .expect("primary window should exist at startup");

    let layout = calculate_hotspot_layout(window.width(), window.height());

    draw_hotspot1(&mut commands, &layout);
    draw_hotspot2(&mut commands, &layout);
    draw_solution_indicator(&mut commands, &layout);
    draw_hotspot4(&mut commands, &layout);
    draw_hotspot5(&mut commands, &layout);
}

fn draw_hotspot1(commands: &mut Commands, layout: &HotspotLayout) {
    let light_blue_color = Color::srgb(0.55, 0.6, 0.9);

    commands
        .spawn((
            Hotspot1,
            Sprite {
                color: light_blue_color,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_left_x, layout.top_row_y, 0.0),
            Pickable::default(),
        ))
        .observe(on_hotspot1_click);
}

fn draw_hotspot2(commands: &mut Commands, layout: &HotspotLayout) {
    let light_red_color = Color::srgb(0.9, 0.6, 0.55);

    commands
        .spawn((
            Hotspot2,
            Sprite {
                color: light_red_color,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_center_x, layout.top_row_y, 0.0),
            Pickable::default(),
        ))
        .observe(open_zoom2);
}

fn draw_solution_indicator(commands: &mut Commands, layout: &HotspotLayout) {
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

fn draw_hotspot4(commands: &mut Commands, layout: &HotspotLayout) {
    let color = Color::srgb(0.7, 0.7, 0.9);

    commands
        .spawn((
            Hotspot4,
            Sprite {
                color,
                custom_size: Some(layout.size),
                ..default()
            },
            Transform::from_xyz(layout.top_left_x, layout.middle_row_y, 0.0),
            Pickable::default(),
        ))
        .observe(on_hotspot4_click);
}

fn draw_hotspot5(commands: &mut Commands, layout: &HotspotLayout) {
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

fn on_hotspot1_click(
    _click: On<Pointer<Click>>,
    selected: Res<SelectedSlot>,
    inventory: Res<Inventory>,
    mut next_zoom: ResMut<NextState<ZoomState>>,
    mut commands: Commands,
) {
    let selected_item = selected
        .index
        .and_then(|index| inventory.slot(index).copied());

    match selected_item {
        Some(ItemId::Key2) => set_game_clear(&mut commands, ClearLabel::Clear2),
        Some(ItemId::Key4) => set_game_clear(&mut commands, ClearLabel::Clear1),
        _ => {
            next_zoom.set(ZoomState::Zoom1);
        }
    }
}

fn set_game_clear(commands: &mut Commands, label: ClearLabel) {
    commands.queue(move |world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(SetGameClear { label })];
        apply_effects(effects, world);
    });
}

fn open_zoom2(_click: On<Pointer<Click>>, mut next_zoom: ResMut<NextState<ZoomState>>) {
    next_zoom.set(ZoomState::Zoom2);
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

fn on_hotspot4_click(
    _click: On<Pointer<Click>>,
    selected: Res<SelectedSlot>,
    inventory: Res<Inventory>,
    mut commands: Commands,
) {
    let Some(index) = selected.index else {
        return;
    };
    let Some(item) = inventory.slot(index) else {
        return;
    };
    if *item != ItemId::Key1 {
        return;
    }

    commands.queue(move |world: &mut World| {
        let effects: Vec<Box<dyn Effect>> =
            vec![Box::new(remove_item_at(index)), Box::new(UnlockRect5)];
        apply_effects(effects, world);
        world.resource_mut::<SelectedSlot>().index = None;
    });
}

fn on_hotspot5_click(_click: On<Pointer<Click>>, mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(GiveItem { item: ItemId::Key3 })];
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
