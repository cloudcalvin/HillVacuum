//=======================================================================//
// IMPORTS
//
//=======================================================================//

use bevy::prelude::Vec2;
use shared::return_if_none;

use super::item_selector::{ItemSelector, ItemsBeneathCursor};
use crate::{
    map::{
        drawer::color::Color,
        editor::{
            cursor_pos::Cursor,
            state::{
                core::{draw_selected_and_non_selected_brushes, ActiveTool},
                editor_state::InputsPresses,
                edits_history::EditsHistory,
                manager::EntitiesManager
            },
            DrawBundle,
            ToolUpdateBundle
        }
    },
    utils::{
        identifiers::{EntityId, Id},
        iterators::FilterSet,
        misc::Camera
    }
};

//=======================================================================//
// TYPES
//
//=======================================================================//

#[derive(Debug)]
struct Selector(ItemSelector<Id>);

impl Selector
{
    #[inline]
    #[must_use]
    fn new() -> Self { Self(ItemSelector::new(Self::selector)) }

    #[inline]
    fn selector(
        manager: &EntitiesManager,
        cursor_pos: Vec2,
        _: f32,
        items: &mut ItemsBeneathCursor<Id>
    )
    {
        for brush in manager
            .selected_brushes_at_pos(cursor_pos, None)
            .iter()
            .filter(|brush| brush.contains_point(cursor_pos))
        {
            items.push(brush.id(), true);
        }
    }

    #[inline]
    #[must_use]
    fn brush_beneath_cursor(
        &mut self,
        manager: &EntitiesManager,
        cursor: &Cursor,
        inputs: &InputsPresses
    ) -> Option<Id>
    {
        self.0.item_beneath_cursor(manager, cursor, 0f32, inputs)
    }
}

//=======================================================================//

#[derive(Debug)]
pub(in crate::map::editor::state::core) struct ShatterTool(Option<Id>, Selector);

impl ShatterTool
{
    #[inline]
    pub fn tool() -> ActiveTool { ActiveTool::Shatter(ShatterTool(None, Selector::new())) }

    #[inline]
    #[must_use]
    const fn cursor_pos(cursor: &Cursor) -> Vec2 { cursor.world_snapped() }

    //==============================================================
    // Update

    #[inline]
    pub fn update(
        &mut self,
        bundle: &mut ToolUpdateBundle,
        manager: &mut EntitiesManager,
        inputs: &InputsPresses,
        edits_history: &mut EditsHistory
    )
    {
        self.0 = self.1.brush_beneath_cursor(manager, bundle.cursor, inputs);

        if inputs.left_mouse.just_pressed()
        {
            self.shatter(bundle, manager, edits_history);
        }
    }

    #[inline]
    fn shatter(
        &mut self,
        bundle: &mut ToolUpdateBundle,
        manager: &mut EntitiesManager,
        edits_history: &mut EditsHistory
    )
    {
        let ToolUpdateBundle { camera, cursor, .. } = bundle;

        let id = return_if_none!(self.0);

        manager.spawn_brushes(
            return_if_none!(manager.brush(id).shatter(
                bundle.drawing_resources,
                Self::cursor_pos(cursor),
                camera.scale()
            )),
            edits_history
        );
        manager.despawn_selected_brush(id, edits_history);

        self.0 = None;
    }

    //==============================================================
    // Draw

    #[inline]
    pub fn draw(&self, bundle: &mut DrawBundle, manager: &EntitiesManager)
    {
        bundle
            .drawer
            .square_highlight(Self::cursor_pos(bundle.cursor), Color::ToolCursor);

        if let Some(hgl_e) = self.0
        {
            manager
                .brush(hgl_e)
                .draw_highlighted_selected(bundle.camera, &mut bundle.drawer);

            for brush in manager
                .visible_brushes(bundle.window, bundle.camera)
                .iter()
                .filter_set_with_predicate(hgl_e, |brush| brush.id())
            {
                if manager.is_selected(brush.id())
                {
                    brush.draw_selected(bundle.camera, &mut bundle.drawer);
                }
                else
                {
                    brush.draw_non_selected(bundle.camera, &mut bundle.drawer);
                }
            }
        }
        else
        {
            draw_selected_and_non_selected_brushes!(bundle, manager);
        }
    }
}
