use bevy::math::Vec2;

/// Geometry for the top-row hotspots, derived from the window size.
pub struct HotspotLayout {
    pub size: Vec2,
    pub top_row_y: f32,
    pub top_left_x: f32,
    pub top_center_x: f32,
    pub top_right_x: f32,
}

pub fn calculate_hotspot_layout(window_width: f32, window_height: f32) -> HotspotLayout {
    let cell_width = window_width / 3.0;
    let cell_height = window_height / 3.0;
    let size = Vec2::new(cell_width * 0.85, cell_height * 0.85);

    let top_row_y = window_height / 2.0 - cell_height / 2.0;
    let top_left_x = -window_width / 2.0 + cell_width / 2.0;
    let top_center_x = 0.0;
    let top_right_x = window_width / 2.0 - cell_width / 2.0;

    HotspotLayout {
        size,
        top_row_y,
        top_left_x,
        top_center_x,
        top_right_x,
    }
}

pub struct InventorySlotLayout {
    pub slot_size: Vec2,
    pub positions: Vec<Vec2>,
}

pub fn calculate_inventory_slot_layout(
    window_width: f32,
    window_height: f32,
    slot_count: usize,
) -> InventorySlotLayout {
    let slot_size = Vec2::new(50.0, 50.0);
    let margin = 10.0;

    let bottom_row_y = -window_height / 2.0 + slot_size.y / 2.0 + margin;
    let rightmost_x = window_width / 2.0 - margin - slot_size.x / 2.0;
    let leftmost_x = rightmost_x - (slot_count as f32 - 1.0) * (slot_size.x + margin);

    let positions = (0..slot_count)
        .map(|index| {
            Vec2::new(
                leftmost_x + index as f32 * (slot_size.x + margin),
                bottom_row_y,
            )
        })
        .collect();

    InventorySlotLayout {
        slot_size,
        positions,
    }
}
