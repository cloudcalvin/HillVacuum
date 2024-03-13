//=======================================================================//
// IMPORTS
//
//=======================================================================//

use bevy::math::UVec2;
use bevy_egui::egui;
use shared::{return_if_none, TEXTURE_HEIGHT_RANGE};

use super::{
    bottom_area,
    tool::{ActiveTool, ChangeConditions, EnabledTool, SubTool}
};
use crate::{
    map::{
        containers::{hv_hash_set, Ids},
        drawer::color::Color,
        editor::{
            state::{
                clipboard::Clipboard,
                core::tool::subtools_buttons,
                editor_state::{InputsPresses, ToolsSettings},
                edits_history::EditsHistory,
                format_texture_preview,
                manager::EntitiesManager,
                ui::{
                    overall_value_field::MinusPlusOverallValueField,
                    textures_gallery,
                    ToolsButtons
                }
            },
            DrawBundle,
            StateUpdateBundle,
            ToolUpdateBundle
        },
        thing::ThingInterface,
        AssertedInsertRemove
    },
    utils::{
        identifiers::{EntityId, Id},
        overall_value::{OverallValue, OverallValueInterface, OverallValueToUi, UiOverallValue}
    }
};

//=======================================================================//
// ENUMS
//
//=======================================================================//

#[derive(Debug)]
enum Status
{
    Inactive(()),
    ChangeUi
}

impl Default for Status
{
    #[inline]
    fn default() -> Self { Self::Inactive(()) }
}

impl EnabledTool for Status
{
    type Item = SubTool;

    #[inline]
    fn is_tool_enabled(&self, tool: Self::Item) -> bool
    {
        tool == match self
        {
            Self::ChangeUi => SubTool::ThingChange,
            Self::Inactive(()) => return false
        }
    }
}

//=======================================================================//
// TYPES
//
//=======================================================================//

#[derive(Debug)]
pub(in crate::map::editor::state::core) struct ThingTool
{
    drawn_things:        Ids,
    max_ui_height:       f32,
    overall_draw_height: UiOverallValue<i8>,
    overall_angle:       UiOverallValue<f32>,
    status:              Status
}

impl ThingTool
{
    #[inline]
    pub fn tool(manager: &EntitiesManager) -> ActiveTool
    {
        ActiveTool::Thing(ThingTool {
            drawn_things:        hv_hash_set![],
            max_ui_height:       0f32,
            overall_draw_height: Self::overall_height(manager),
            overall_angle:       Self::overall_angle(manager),
            status:              Status::default()
        })
    }

    #[inline]
    fn overall_height(manager: &EntitiesManager) -> UiOverallValue<i8>
    {
        let mut overall = OverallValue::None;

        for thing in manager.selected_things()
        {
            _ = overall.stack(&thing.draw_height());
        }

        overall.ui()
    }

    #[inline]
    fn overall_angle(manager: &EntitiesManager) -> UiOverallValue<f32>
    {
        let mut overall = OverallValue::None;

        for thing in manager.selected_things()
        {
            _ = overall.stack(&thing.angle());
        }

        overall.ui()
    }

    #[inline]
    pub fn update_overall_thing_info(&mut self, manager: &EntitiesManager)
    {
        self.overall_draw_height = Self::overall_height(manager);
        self.overall_angle = Self::overall_angle(manager);
    }

    #[inline]
    pub fn disable_subtool(&mut self) { self.status = Status::Inactive(()); }

    #[inline]
    pub fn update(
        &mut self,
        bundle: &ToolUpdateBundle,
        manager: &mut EntitiesManager,
        inputs: &InputsPresses,
        edits_history: &mut EditsHistory,
        settings: &mut ToolsSettings
    )
    {
        if !matches!(self.status, Status::Inactive(()))
        {
            return;
        }

        if inputs.left_mouse.just_pressed()
        {
            self.drawn_things.asserted_insert(manager.spawn_selected_thing(
                bundle.things_catalog,
                edits_history,
                settings.thing_pivot.spawn_pos(
                    bundle.things_catalog.selected_thing(),
                    bundle.cursor.world_snapped()
                )
            ));
        }
        else if inputs.back.just_pressed()
        {
            manager.despawn_drawn_things(&mut self.drawn_things, edits_history);
        }
        else if inputs.tab.just_pressed()
        {
            if inputs.shift_pressed()
            {
                settings.thing_pivot.prev();
            }
            else
            {
                settings.thing_pivot.next();
            }
        }
    }

    #[inline]
    pub fn undo_redo_spawn(&mut self, manager: &EntitiesManager, identifier: Id)
    {
        assert!(manager.entity_exists(identifier), "Entity does not exist.");
        self.drawn_things.asserted_insert(identifier);
    }

    #[inline]
    pub fn undo_redo_despawn(&mut self, manager: &EntitiesManager, identifier: Id)
    {
        assert!(!manager.entity_exists(identifier), "Entity exists.");
        self.drawn_things.asserted_remove(&identifier);
    }

    #[inline]
    pub fn draw(&self, bundle: &mut DrawBundle, manager: &EntitiesManager)
    {
        let DrawBundle {
            drawer,
            window,
            camera,
            things_catalog,
            ..
        } = bundle;

        drawer.square_highlight(bundle.cursor.world_snapped(), Color::CursorPolygon);

        let mut iterated_drawn = 0;
        let drawn_len = self.drawn_things.len();
        let things = manager.visible_things(window, camera);
        let mut things = things.iter();

        for thing in things.by_ref()
        {
            let id = thing.id();

            if !manager.is_selected(id)
            {
                thing.draw_non_selected(drawer, things_catalog);
            }
            else if self.drawn_things.contains(&id)
            {
                thing.draw_highlighted_selected(drawer, things_catalog);
                iterated_drawn += 1;

                if iterated_drawn == drawn_len
                {
                    break;
                }
            }
            else
            {
                thing.draw_selected(drawer, things_catalog);
            }
        }

        for thing in things
        {
            let id = thing.id();

            if manager.is_selected(id)
            {
                thing.draw_selected(drawer, things_catalog);
            }
            else
            {
                thing.draw_non_selected(drawer, things_catalog);
            }
        }

        for brush in manager.visible_brushes(window, camera).iter()
        {
            brush.draw_opaque(camera, drawer);
        }
    }

    #[inline]
    #[must_use]
    pub fn left_panel(
        &mut self,
        ui: &mut egui::Ui,
        manager: &mut EntitiesManager,
        inputs: &InputsPresses,
        edits_history: &mut EditsHistory,
        clipboard: &mut Clipboard,
        settings: &mut ToolsSettings
    ) -> bool
    {
        const LABEL_WIDTH: f32 = 50f32;

        ui.spacing_mut().item_spacing.x = 2f32;
        let mut has_focus = false;

        macro_rules! overall_value {
            ($strip:ident, $label:literal, $value:ident, $t:ty, $min:expr, $max:expr) => {
                paste::paste! {
                    $strip.strip(|strip| {
                        strip
                            .size(egui_extras::Size::exact(LABEL_WIDTH))
                            .size(egui_extras::Size::exact(64f32))
                            .size(egui_extras::Size::remainder())
                            .horizontal(|mut strip| {
                                const ONE: $t = 1 as $t;

                                strip.cell(|ui| {
                                    ui.label($label);
                                });

                                has_focus = MinusPlusOverallValueField::new((16f32, 19f32).into())
                                    .show(
                                        &mut strip,
                                        clipboard,
                                        inputs,
                                        &mut self.[< overall_ $value >],
                                        ONE,
                                        |value: $t, _| value.clamp($min, $max),
                                        |value| {
                                            edits_history.[< thing_ $value _cluster >](
                                                manager.selected_things_mut().filter_map(
                                                    |mut thing| {
                                                        thing
                                                            .[< set_ $value >](value)
                                                            .map(|value| (thing.id(), value))
                                                    }
                                                )
                                            );

                                            value.into()
                                        }
                                    )
                                    .has_focus;
                            });
                    });
                }
            };
        }

        egui_extras::StripBuilder::new(ui)
            .sizes(egui_extras::Size::exact(18f32), 4)
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    ui.label(egui::RichText::new("THING TOOL"));
                });

                strip.strip(|strip| {
                    strip
                        .size(egui_extras::Size::exact(LABEL_WIDTH))
                        .size(egui_extras::Size::remainder())
                        .horizontal(|mut strip| {
                            settings.thing_pivot.ui(&mut strip);
                        });
                });

                overall_value!(
                    strip,
                    "Height",
                    draw_height,
                    i8,
                    *TEXTURE_HEIGHT_RANGE.start(),
                    *TEXTURE_HEIGHT_RANGE.end()
                );
                overall_value!(strip, "Angle", angle, f32, 0f32, 360f32);
            });

        has_focus
    }

    #[allow(clippy::cast_precision_loss)]
    #[inline]
    pub fn bottom_panel(
        &mut self,
        bundle: &mut StateUpdateBundle,
        manager: &mut EntitiesManager,
        inputs: &InputsPresses,
        edits_history: &mut EditsHistory
    )
    {
        const PREVIEW_SIZE: egui::Vec2 = egui::Vec2::splat(128f32);

        let StateUpdateBundle {
            things_catalog,
            egui_context,
            drawing_resources,
            ..
        } = bundle;

        let clicked = return_if_none!(bottom_area!(
            self,
            egui_context,
            things_catalog,
            "things",
            thing,
            PREVIEW_SIZE.y + 28f32,
            PREVIEW_SIZE,
            |ui: &mut egui::Ui,
             texture: (usize, egui::TextureId, UVec2, &str),
             frame: egui::Vec2| {
                ui.vertical(|ui| {
                    ui.set_width(frame.x);

                    let response =
                        format_texture_preview!(ImageButton, ui, texture.1, texture.2, frame.x);
                    ui.vertical_centered(|ui| {
                        ui.label(texture.3);
                    });
                    response
                })
                .inner
            },
            str,
            drawing_resources
        ));

        if !inputs.alt_pressed() && !matches!(self.status, Status::ChangeUi)
        {
            things_catalog.set_selected_thing_index(clicked);
            return;
        }

        self.status = Status::Inactive(());

        let clicked = things_catalog.thing_at_index(clicked);
        let valid = manager.test_operation_validity(|manager| {
            manager
                .selected_things()
                .find_map(|thing| (!thing.check_thing_change(clicked)).then_some(thing.id()))
        });

        if !valid
        {
            return;
        }

        edits_history.thing_change_cluster(
            manager
                .selected_things_mut()
                .filter_map(|mut thing| thing.set_thing(clicked).map(|prev| (thing.id(), prev)))
        );
    }

    #[inline]
    pub fn draw_sub_tools(
        &mut self,
        ui: &mut egui::Ui,
        bundle: &StateUpdateBundle,
        buttons: &mut ToolsButtons,
        tool_change_conditions: &ChangeConditions
    )
    {
        subtools_buttons!(
            self.status,
            ui,
            bundle,
            buttons,
            tool_change_conditions,
            (ThingChange, Status::ChangeUi, Status::ChangeUi)
        );
    }
}
