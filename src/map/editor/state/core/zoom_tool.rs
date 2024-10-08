//=======================================================================//
// IMPORTS
//
//=======================================================================//

use hill_vacuum_shared::return_if_none;

use super::{
    rect::{Rect, RectTrait},
    tool::DragSelection,
    ActiveTool,
    PreviousActiveTool
};
use crate::{
    map::{
        drawer::color::Color,
        editor::{state::core::rect, DrawBundle, ToolUpdateBundle}
    },
    utils::{collections::hv_box, misc::Camera}
};

//=======================================================================//
// STRUCTS
//
//=======================================================================//

/// The tool used to zoom in/out the map view.
pub(in crate::map::editor::state::core) struct ZoomTool
{
    /// The drag selection.
    drag_selection:           Rect,
    /// The tool that was being used before enabling the zoom tool.
    pub previous_active_tool: PreviousActiveTool
}

impl DragSelection for ZoomTool
{
    #[inline]
    fn drag_selection(&self) -> Option<Rect> { self.drag_selection.into() }
}

impl ZoomTool
{
    /// Returns a new [`ActiveTool`] in its zoom tool variant.
    #[inline]
    pub fn tool(drag_selection: Rect, active_tool: &mut ActiveTool) -> ActiveTool
    {
        ActiveTool::Zoom(Self {
            drag_selection,
            previous_active_tool: hv_box!(std::mem::take(active_tool))
        })
    }

    /// Updates the tool.
    #[allow(unreachable_code)]
    #[inline]
    pub fn update<'a>(
        &'a mut self,
        bundle: &mut ToolUpdateBundle
    ) -> Option<&'a mut PreviousActiveTool>
    {
        let ToolUpdateBundle {
            window,
            camera,
            cursor,
            inputs,
            grid,
            ..
        } = bundle;

        rect::update!(
            self.drag_selection,
            cursor.world_snapped(),
            inputs.left_mouse.pressed(),
            inputs.left_mouse.just_pressed(),
            {
                return Some(&mut self.previous_active_tool);
            },
            hull,
            {
                camera.scale_viewport_to_hull(window, grid, &hull, 0f32);
                return Some(&mut self.previous_active_tool);
            }
        );

        None
    }

    /// Draws the tool.
    #[inline]
    pub fn draw(&self, bundle: &mut DrawBundle)
    {
        let DrawBundle { drawer, .. } = bundle;
        drawer.hull(&return_if_none!(self.drag_selection.hull()), Color::Hull);
    }
}
