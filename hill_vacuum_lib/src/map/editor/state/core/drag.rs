//=======================================================================//
// IMPORTS
//
//=======================================================================//

use bevy::prelude::{Transform, Vec2, Window};
use bevy_egui::egui;
use shared::return_if_none;

use crate::{
    map::{
        drawer::{color::Color, EditDrawer},
        editor::{cursor_pos::Cursor, state::grid::Grid}
    },
    utils::{
        math::AroundEqual,
        tooltips::{
            draw_tooltip_x_centered_above_pos,
            draw_tooltip_y_centered,
            to_egui_coordinates
        }
    }
};

//=======================================================================//
// STRUCTS
//
//=======================================================================//

#[derive(Clone, Copy, Debug)]
pub(in crate::map) struct Drag
{
    origin: Vec2,
    delta:  Vec2
}

impl Drag
{
    pub(in crate::map) const X_DRAG: &'static str = "x_drag";
    pub(in crate::map) const Y_DRAG: &'static str = "y_drag";

    #[inline]
    #[must_use]
    pub(in crate::map::editor::state::core) const fn new(origin: Vec2) -> Self
    {
        Self {
            origin,
            delta: Vec2::ZERO
        }
    }

    #[inline]
    #[must_use]
    pub(in crate::map::editor::state::core) fn try_new_initiated(
        origin: Vec2,
        cursor: &Cursor,
        grid: Grid
    ) -> Option<Self>
    {
        let drag = Self::new(origin);
        drag.overall_and_frame_drag_delta_from_origin(cursor, grid)
            .is_some()
            .then_some(drag)
    }

    #[inline]
    #[must_use]
    pub(in crate::map::editor::state::core) const fn delta(&self) -> Vec2 { self.delta }

    #[inline]
    pub(in crate::map::editor::state::core) fn conditional_update<F: FnMut(Vec2) -> bool>(
        &mut self,
        cursor: &Cursor,
        grid: Grid,
        mut dragger: F
    )
    {
        let (overall_delta, frame_delta) =
            return_if_none!(self.overall_and_frame_drag_delta_from_origin(cursor, grid));

        if dragger(frame_delta)
        {
            self.delta = overall_delta;
        }
    }

    #[inline]
    pub(in crate::map::editor::state::core) fn update<F: FnMut(Vec2)>(
        &mut self,
        cursor: &Cursor,
        grid: Grid,
        mut dragger: F
    )
    {
        let (overall_delta, frame_delta) =
            return_if_none!(self.overall_and_frame_drag_delta_from_origin(cursor, grid));

        dragger(frame_delta);
        self.delta = overall_delta;
    }

    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn overall_and_frame_drag_delta_from_origin(
        &self,
        cursor: &Cursor,
        grid: Grid
    ) -> Option<(Vec2, Vec2)>
    {
        let snap = cursor.snap();
        let cursor_pos = cursor.world_snapped();
        let prev_step = self.origin + self.delta;

        let delta = if snap
        {
            let delta = cursor_pos - prev_step;
            let target = prev_step + grid.square(delta).nearest_corner_to_point(delta);

            if prev_step.around_equal(&target)
            {
                return None;
            }

            (target - self.origin).round()
        }
        else
        {
            if prev_step.around_equal(&cursor_pos)
            {
                return None;
            }

            cursor_pos - self.origin
        };

        Some((delta, delta - self.delta))
    }

    #[inline]
    pub(in crate::map::editor::state::core) fn draw(
        &self,
        window: &Window,
        camera: &Transform,
        egui_context: &egui::Context,
        drawer: &mut EditDrawer
    )
    {
        const TOOLTIP_TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(127, 255, 212);

        if self.delta.around_equal_narrow(&Vec2::ZERO)
        {
            return;
        }

        let p = self.origin + self.delta;
        drawer.line(self.origin, Vec2::new(p.x, self.origin.y), Color::Hull);
        drawer.line(Vec2::new(p.x, self.origin.y), p, Color::Hull);

        if self.delta.x != 0f32
        {
            draw_tooltip_x_centered_above_pos(
                egui_context,
                Self::X_DRAG,
                egui::Order::Background,
                #[allow(clippy::cast_possible_truncation)]
                &format!("{}", self.delta.x as i16),
                egui::TextStyle::Monospace,
                to_egui_coordinates(
                    Vec2::new(self.origin.x + self.delta.x / 2f32, self.origin.y),
                    window,
                    camera
                ),
                egui::Vec2::new(0f32, -4f32),
                TOOLTIP_TEXT_COLOR,
                egui::Color32::TRANSPARENT,
                0f32
            );
        }

        if self.delta.y == 0f32
        {
            return;
        }

        draw_tooltip_y_centered(
            egui_context,
            Self::Y_DRAG,
            egui::Order::Background,
            #[allow(clippy::cast_possible_truncation)]
            format!("{}", self.delta.y as i8).as_str(),
            egui::TextStyle::Monospace,
            to_egui_coordinates(Vec2::new(p.x, p.y - self.delta.y / 2f32), window, camera),
            egui::Vec2::new(4f32, 0f32),
            TOOLTIP_TEXT_COLOR,
            egui::Color32::from_black_alpha(0),
            0f32
        );
    }
}
