use bevy::app::App;
use bevy_escape_core::{Effect, InventoryFullMessage, apply_effects};
use bevy_escape_template::domain::clear::{ClearLabel, GameClear, SetGameClear};
use bevy_escape_template::domain::item::{
    GiveItem, INVENTORY_CAPACITY, Inventory, ItemId, remove_item_at,
};
use bevy_escape_template::domain::progress::{Progress, UnlockRect5};
use bevy_escape_template::domain::selection::{SelectedSlot, SlotClickOutcome, resolve_slot_click};
use bevy_escape_template::domain::solved::{
    Solution1, Solved, WrongAnswerMessage, resolve_quiz1_effects,
};

fn test_app() -> App {
    let mut app = App::new();
    app.init_resource::<Solved>();
    app.init_resource::<Progress>();
    app.init_resource::<SelectedSlot>();
    app.init_resource::<GameClear>();
    app.insert_resource(Inventory::new(INVENTORY_CAPACITY));
    app.add_message::<WrongAnswerMessage>();
    app.add_message::<InventoryFullMessage<ItemId>>();
    app
}

#[test]
fn full_playthrough_reaches_clear2_via_key2() {
    let mut app = test_app();

    // Solve quiz1 with "12" -> Answer12 (purple).
    apply_effects(resolve_quiz1_effects("12"), app.world_mut());
    assert_eq!(app.world().resource::<Solved>().quiz1, Solution1::Answer12);

    // Click rect3 (purple) -> give key2.
    apply_effects(
        vec![Box::new(GiveItem { item: ItemId::Key2 })],
        app.world_mut(),
    );
    assert!(app.world().resource::<Inventory>().has(&ItemId::Key2));

    // Select key2's slot, then "click rect1" -> game clear (clear2).
    app.world_mut().resource_mut::<SelectedSlot>().index = Some(0);
    apply_effects(
        vec![Box::new(SetGameClear {
            label: ClearLabel::Clear2,
        })],
        app.world_mut(),
    );

    assert_eq!(
        app.world().resource::<GameClear>().label,
        Some(ClearLabel::Clear2)
    );
}

#[test]
fn combining_two_key3s_then_using_key4_reaches_clear1() {
    let mut app = test_app();

    apply_effects(
        vec![Box::new(GiveItem { item: ItemId::Key3 })],
        app.world_mut(),
    );
    apply_effects(
        vec![Box::new(GiveItem { item: ItemId::Key3 })],
        app.world_mut(),
    );

    {
        let inventory = app.world().resource::<Inventory>();
        assert_eq!(inventory.slot(0), Some(&ItemId::Key3));
        assert_eq!(inventory.slot(1), Some(&ItemId::Key3));
    }

    app.world_mut().resource_mut::<SelectedSlot>().index = Some(0);

    let outcome = {
        let inventory = app.world().resource::<Inventory>();
        let selected = app.world().resource::<SelectedSlot>();
        resolve_slot_click(1, inventory, selected)
    };
    assert_eq!(
        outcome,
        SlotClickOutcome::Combine {
            selected_index: 0,
            clicked_index: 1,
            result: ItemId::Key4,
        }
    );

    let effects: Vec<Box<dyn Effect>> = vec![
        Box::new(remove_item_at::<ItemId>(0)),
        Box::new(remove_item_at::<ItemId>(1)),
        Box::new(GiveItem { item: ItemId::Key4 }),
    ];
    apply_effects(effects, app.world_mut());
    app.world_mut().resource_mut::<SelectedSlot>().index = None;

    {
        let inventory = app.world().resource::<Inventory>();
        assert_eq!(inventory.slot(0), Some(&ItemId::Key4));
        assert_eq!(inventory.slot(1), None);
    }

    // Select key4, then "click rect1" -> game clear (clear1).
    app.world_mut().resource_mut::<SelectedSlot>().index = Some(0);
    apply_effects(
        vec![Box::new(SetGameClear {
            label: ClearLabel::Clear1,
        })],
        app.world_mut(),
    );

    assert_eq!(
        app.world().resource::<GameClear>().label,
        Some(ClearLabel::Clear1)
    );
}

#[test]
fn using_key1_on_hotspot4_consumes_it_and_unlocks_rect5() {
    let mut app = test_app();
    apply_effects(
        vec![Box::new(GiveItem { item: ItemId::Key1 })],
        app.world_mut(),
    );

    let effects: Vec<Box<dyn Effect>> =
        vec![Box::new(remove_item_at::<ItemId>(0)), Box::new(UnlockRect5)];
    apply_effects(effects, app.world_mut());

    assert_eq!(app.world().resource::<Inventory>().slot(0), None);
    assert!(app.world().resource::<Progress>().rect5_unlocked);
}

#[test]
fn a_slot_freed_by_one_item_can_be_reused_by_a_different_item_type() {
    let mut app = test_app();
    apply_effects(
        vec![Box::new(GiveItem { item: ItemId::Key1 })],
        app.world_mut(),
    );
    apply_effects(vec![Box::new(remove_item_at::<ItemId>(0))], app.world_mut());
    apply_effects(
        vec![Box::new(GiveItem { item: ItemId::Key3 })],
        app.world_mut(),
    );

    assert_eq!(
        app.world().resource::<Inventory>().slot(0),
        Some(&ItemId::Key3)
    );
}
