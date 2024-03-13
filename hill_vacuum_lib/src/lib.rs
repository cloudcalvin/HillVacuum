#![allow(clippy::single_match_else)]
#![cfg_attr(feature = "arena_alloc", feature(allocator_api))]

mod config;
mod embedded_assets;
mod map;
mod utils;

//=======================================================================//
// IMPORTS
//
//=======================================================================//

use bevy::{
    a11y::AccessibilityPlugin,
    core_pipeline::CorePipelinePlugin,
    input::InputPlugin,
    prelude::*,
    render::{
        pipelined_rendering::PipelinedRenderingPlugin,
        texture::{ImageAddressMode, ImageSamplerDescriptor},
        RenderPlugin
    },
    sprite::SpritePlugin,
    time::TimePlugin,
    window::Cursor,
    winit::WinitPlugin
};
use config::ConfigPlugin;
use embedded_assets::EmbeddedPlugin;
use map::MapEditorPlugin;
use proc_macros::str_array;

//=======================================================================//
// EXPORTS
//
//=======================================================================//
pub use crate::map::{
    brush::{
        mover::{Motor, Mover},
        BrushViewer as Brush
    },
    drawer::{
        animation::{Animation, Atlas, List},
        texture::{Sprite, TextureInterface, TextureSettings}
    },
    path::{
        nodes::{Movement, Node},
        Path
    },
    thing::{catalog::HardcodedThings, MapThing, Thing, ThingViewer as ThingInstance},
    Exporter
};

//=======================================================================//
// CONSTANTS
//
//=======================================================================//

/// The name of the application.
const NAME: &str = "HillVacuum";
/// The folder where the assets are stored.
const ASSETS_PATH: &str = "assets/";
str_array!(INDEXES, 128);

const PROP_CAMERAS_ROWS: usize = 2;
const PROP_CAMERAS_AMOUNT: usize = 8 * (PROP_CAMERAS_ROWS * (PROP_CAMERAS_ROWS + 1)) / 2;

//=======================================================================//
// ENUMS
//
//=======================================================================//

/// The overall states of the application.
#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
enum EditorState
{
    /// Boot.
    #[default]
    SplashScreen,
    /// Program running.
    Run,
    /// Shutdown procedure.
    ShutDown
}

//=======================================================================//

/// Actions with hardcoded key binds.
enum HardcodedActions
{
    /// New file.
    New,
    /// Save file.
    Save,
    /// Open file.
    Open,
    /// Export file.
    Export,
    /// Select all.
    SelectAll,
    /// Copy.
    Copy,
    /// Paste.
    Paste,
    /// Cut.
    Cut,
    /// Duplicate.
    Duplicate,
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Camera zoom in.
    ZoomIn,
    /// Camera zoom out.
    ZoomOut,
    /// Toggle fullscreen view.
    Fullscreen,
    /// Toggle the manual.
    ToggleManual,
    /// Quit.
    Quit
}

impl HardcodedActions
{
    /// A string representation of the key presses required to initiate the action.
    #[inline]
    #[must_use]
    pub const fn key_combo(self) -> &'static str
    {
        match self
        {
            Self::New => "Ctrl+N",
            Self::Save => "Ctrl+S",
            Self::Open => "Ctrl+O",
            Self::Export => "Ctrl+E",
            Self::SelectAll => "Ctrl+A",
            Self::Copy => "Ctrl+C",
            Self::Paste => "Ctrl+V",
            Self::Cut => "Ctrl+X",
            Self::Duplicate => "Ctrl+D",
            Self::Undo => "Ctrl+Z",
            Self::Redo => "Ctrl+Y",
            Self::ZoomIn => "Ctrl+Plus",
            Self::ZoomOut => "Ctrl+Minus",
            Self::Fullscreen => "Alt+Enter",
            Self::ToggleManual => "`",
            Self::Quit => "Ctrl+Q"
        }
    }

    /// Returns the [`Keycode`] associated to the action.
    #[inline]
    #[must_use]
    pub const fn key(self) -> KeyCode
    {
        match self
        {
            Self::New => KeyCode::KeyN,
            Self::Save => KeyCode::KeyS,
            Self::Open => KeyCode::KeyO,
            Self::Export => KeyCode::KeyE,
            Self::Fullscreen => KeyCode::Enter,
            Self::ToggleManual => KeyCode::Backquote,
            Self::SelectAll => KeyCode::KeyA,
            Self::Copy => KeyCode::KeyC,
            Self::Paste => KeyCode::KeyV,
            Self::Cut => KeyCode::KeyX,
            Self::Duplicate => KeyCode::KeyD,
            Self::Undo => KeyCode::KeyZ,
            Self::Redo => KeyCode::KeyY,
            Self::ZoomIn => KeyCode::NumpadAdd,
            Self::ZoomOut => KeyCode::Minus,
            Self::Quit => KeyCode::KeyQ
        }
    }

    /// Whever the action's keys were pressed.
    #[inline]
    #[must_use]
    pub fn pressed(self, key_inputs: &ButtonInput<KeyCode>) -> bool
    {
        match self
        {
            Self::Fullscreen =>
            {
                return (key_inputs.pressed(KeyCode::AltLeft) ||
                    key_inputs.pressed(KeyCode::AltRight)) &&
                    key_inputs.just_pressed(self.key())
            },
            Self::ToggleManual => return key_inputs.just_pressed(self.key()),
            _ => ()
        };

        if !(key_inputs.pressed(KeyCode::ControlLeft) || key_inputs.pressed(KeyCode::ControlRight))
        {
            return false;
        }

        key_inputs.just_pressed(self.key())
    }
}

//=======================================================================//
// TYPES
//
//=======================================================================//

/// The main plugin.
pub struct HillVacuumPlugin;

impl Plugin for HillVacuumPlugin
{
    #[inline]
    fn build(&self, app: &mut App)
    {
        app.add_plugins((
            AssetPlugin {
                file_path: ASSETS_PATH.to_owned(),
                processed_file_path: "processed_assets/".to_owned(),
                watch_for_changes_override: false.into(),
                mode: bevy::prelude::AssetMode::Unprocessed
            },
            AccessibilityPlugin,
            TaskPoolPlugin::default(),
            TypeRegistrationPlugin,
            FrameCountPlugin,
            TimePlugin,
            TransformPlugin,
            InputPlugin,
            WindowPlugin {
                primary_window: Some(Window {
                    cursor: Cursor {
                        icon: CursorIcon::Pointer,
                        ..Default::default()
                    },
                    title: NAME.into(),
                    position: WindowPosition::At((0, 0).into()),
                    resolution: (1920f32, 1080f32).into(),
                    resize_constraints: WindowResizeConstraints {
                        min_width: 640f32,
                        min_height: 480f32,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            WinitPlugin {
                run_on_any_thread: true
            },
            RenderPlugin::default(),
            ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                }
            }
        ));

        #[cfg(not(target_arch = "wasm32"))]
        {
            app.add_plugins(PipelinedRenderingPlugin);
        }

        app.add_plugins((CorePipelinePlugin, SpritePlugin));

        #[cfg(feature = "debug")]
        {
            app.add_plugins(bevy::gizmos::GizmoPlugin);
        }

        app.add_plugins((EmbeddedPlugin, ConfigPlugin, MapEditorPlugin))
            .insert_state(EditorState::default())
            .insert_resource(Msaa::Sample4);
    }
}
