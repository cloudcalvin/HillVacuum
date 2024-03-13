//=======================================================================//
// IMPORTS
//
//=======================================================================//

use super::{tool::ActiveTool, PreviousActiveTool};
use crate::{
    map::{
        containers::{hv_box, HvVec},
        drawer::drawing_resources::DrawingResources,
        editor::{
            state::manager::{Animators, EntitiesManager},
            DrawBundleMapPreview,
            ToolUpdateBundle
        },
        path::MovementSimulator
    },
    utils::identifiers::{EntityId, Id}
};

//=======================================================================//
// TYPES
//
//=======================================================================//

#[derive(Debug)]
pub(in crate::map::editor::state::core) struct MapPreviewTool
{
    prev_tool: PreviousActiveTool,
    movement:  HvVec<MovementSimulator>,
    animators: Animators
}

impl MapPreviewTool
{
    #[inline]
    pub fn tool(
        drawing_resources: &DrawingResources,
        active_tool: &mut ActiveTool,
        manager: &EntitiesManager
    ) -> ActiveTool
    {
        ActiveTool::MapPreview(MapPreviewTool {
            prev_tool: hv_box!(std::mem::take(active_tool)),
            movement:  manager.movement_simulators(),
            animators: manager.texture_animators(drawing_resources)
        })
    }

    #[inline]
    pub fn prev_tool(&mut self) -> &mut ActiveTool { &mut self.prev_tool }

    #[inline]
    pub fn update(&mut self, bundle: &ToolUpdateBundle, manager: &EntitiesManager)
    {
        for sim in &mut self.movement
        {
            sim.update(manager.brush(sim.id()), bundle.delta_time);
        }

        self.animators
            .update(bundle.drawing_resources, manager, bundle.delta_time);
    }

    #[inline]
    pub fn draw(&self, bundle: &mut DrawBundleMapPreview, manager: &EntitiesManager)
    {
        let DrawBundleMapPreview {
            window,
            drawer,
            camera,
            things_catalog,
            ..
        } = bundle;
        let brushes = manager.brushes();

        for simulator in &self.movement
        {
            manager.moving(simulator.id()).draw_map_preview_movement_simulation(
                camera,
                brushes,
                things_catalog,
                drawer,
                &self.animators,
                simulator
            );
        }

        for brush in manager
            .visible_brushes(window, camera)
            .iter()
            .filter(|brush| !is_moving(manager, brush.id()) && !brush.has_sprite())
        {
            brush.draw_map_preview(camera, drawer, self.animators.get(brush.id()));
        }

        for brush in manager
            .visible_sprites(window, camera)
            .iter()
            .filter(|brush| !is_moving(manager, brush.id()))
        {
            brush.draw_map_preview_sprite(drawer, self.animators.get(brush.id()));
        }

        for thing in manager
            .visible_things(window, camera)
            .iter()
            .filter(|brush| !is_moving(manager, brush.id()))
        {
            thing.draw_map_preview(drawer, things_catalog);
        }
    }
}

//=======================================================================//
// FUNCTIONS
//
//=======================================================================//

#[inline]
#[must_use]
fn is_moving(manager: &EntitiesManager, identifier: Id) -> bool
{
    let moving = manager.is_moving(identifier);

    if manager.is_thing(identifier)
    {
        return moving;
    }

    moving || manager.brush(identifier).anchored().is_some()
}
