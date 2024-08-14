//=======================================================================//
// IMPORTS
//
//=======================================================================//

use bevy::{
    asset::{Assets, Handle},
    color::{Alpha, Srgba},
    sprite::ColorMaterial
};
use bevy_egui::egui;
use configparser::ini::Ini;
use hashbrown::HashMap;
use hill_vacuum_proc_macros::{color_enum, EnumFromUsize, EnumIter, EnumSize};
use hill_vacuum_shared::{match_or_panic, return_if_none};

use super::BevyColor;
use crate::{config::IniConfig, utils::containers::hv_vec};

//=======================================================================//
// CONSTANTS
//
//=======================================================================//

/// The name of the section of the .ini config containing the color settings.
const INI_SECTION: &str = "COLORS";

//=======================================================================//
// TRAITS
//
//=======================================================================//

/// A trait to generate a value from and rbg array.
trait FromArray
{
    /// Returns a new `Self` instance from an rgb array.
    fn from_array(rgb: &[f32; 3]) -> Self;
}

impl FromArray for BevyColor
{
    #[inline]
    #[must_use]
    fn from_array(rgb: &[f32; 3]) -> Self { Self::srgb(rgb[0], rgb[1], rgb[2]) }
}

impl FromArray for egui::Color32
{
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    #[must_use]
    fn from_array(rgb: &[f32; 3]) -> Self
    {
        Self::from_rgb((255f32 * rgb[0]) as u8, (255f32 * rgb[1]) as u8, (255f32 * rgb[2]) as u8)
    }
}

//=======================================================================//

trait Rgb
{
    #[must_use]
    fn to_rgb(&self) -> [f32; 3];
}

impl Rgb for BevyColor
{
    #[inline]
    fn to_rgb(&self) -> [f32; 3]
    {
        match_or_panic!(
            self,
            BevyColor::Srgba(Srgba {
                red,
                green,
                blue,
                ..
            }),
            [*red, *green, *blue]
        )
    }
}

//=======================================================================//
// ENUMS
//
//=======================================================================//

/// The colors used by the map editor.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, EnumSize, EnumFromUsize)]
pub(crate) enum Color
{
    /// The background color.
    Clear,
    /// The color of the grid lines.
    GridLines,
    /// The color of the lines representing the x and y axis.
    OriginGridLines,
    /// The color of the softer colored grids.
    SoftGridLines,
    /// The color of the brushes that are not selected.
    NonSelectedEntity,
    /// The color of the selected brushes.
    SelectedEntity,
    /// The color the highlighted non selected brush.
    HighlightedNonSelectedEntity,
    /// The color of the highlighted selected brush.
    HighlightedSelectedEntity,
    /// The color of brushes that are not relevant to the purposes of the tool being used.
    OpaqueEntity,
    /// The color of the non selected vertexes.
    NonSelectedVertex,
    /// The color of the selected vertexes.
    SelectedVertex,
    /// The color of the brush sides while using the side tool.
    SideModeVertex,
    /// The color of the brushes to which the subtraction is being applied.
    SubtracteeBrush,
    /// The color of the brush that will be subtracted from the other selected brushes.
    SubtractorBrush,
    /// The color of the brushes generated by the clip tool that will be spawned in the map.
    ClippedPolygonsToSpawn,
    /// The color of the brushes generated by the clip tool that will not be spawned in the map.
    ClippedPolygonsNotToSpawn,
    /// The color of the path tool path nodes.
    PathNode,
    /// The color of the selected path tool path node.
    SelectedPathNode,
    /// The color of the highlighted path.
    HighlightedPath,
    /// The color of the lines showing the brushes tied together.
    BrushAnchor,
    /// The color of the lines showing the brushes tied together.
    SpriteAnchor,
    /// The color of the [`Hull`]s' outlines.
    Hull,
    /// The color of the selected brush hull lines extensions.
    HullExtensions,
    /// The color of the cursor.
    DefaultCursor,
    /// The generic color used for the cursor by some tools.
    ToolCursor,
    /// The color of the cursor polygons of the draw tools.
    CursorPolygon,
    /// The color of the [`Hull`] of the cursor cursor polygons of the draw tools.
    CursorPolygonHull,
    /// The color drawn on top of an entity that caused an edit to fail.
    ErrorHighlight
}

impl Color
{
    color_enum!(
        clear: Clear,
        extensions: HullExtensions,
        grid: SoftGridLines,
        GridLines,
        OriginGridLines,
        entities: ClippedPolygonsNotToSpawn | OpaqueEntity,
        NonSelectedEntity,
        SelectedEntity,
        SubtracteeBrush,
        SubtractorBrush,
        ClippedPolygonsToSpawn,
        HighlightedSelectedEntity | HighlightedNonSelectedEntity,
        ui: NonSelectedVertex,
        SelectedVertex,
        SideModeVertex,
        BrushAnchor,
        SpriteAnchor,
        PathNode,
        HighlightedPath,
        SelectedPathNode,
        Hull,
        CursorPolygonHull,
        DefaultCursor,
        ToolCursor | CursorPolygon,
        ErrorHighlight
    );

    /// Returns an iterator to the [`Color`]s that can be customized.
    #[inline]
    fn customizable_colors() -> impl Iterator<Item = Color> { Color::iter().skip(1) }

    /// Returns a [`String`] containing the default colors configuration.
    #[inline]
    #[must_use]
    pub fn default_colors() -> String
    {
        let mut config = String::new();
        config.push_str(&format!("[{INI_SECTION}]\n"));

        for color in Self::customizable_colors()
        {
            config.push_str(&format!(
                "{} = {}\n",
                color.config_file_key(),
                ColorWrapper(color.default_bevy_color())
            ));
        }

        config
    }

    /// The [`BevyColor`] associated with `self`.
    #[inline]
    #[must_use]
    pub const fn default_bevy_color(self) -> BevyColor
    {
        use bevy::color::palettes::css;

        match self
        {
            Self::Clear => BevyColor::Srgba(css::BLACK),
            Self::SoftGridLines => BevyColor::srgb(0.12, 0.2, 0.3),
            Self::GridLines | Self::ClippedPolygonsNotToSpawn => BevyColor::srgb(0.3, 0.3, 0.3),
            Self::OriginGridLines => BevyColor::Srgba(css::WHITE),
            Self::HullExtensions => BevyColor::Srgba(css::INDIGO),
            Self::NonSelectedEntity => BevyColor::Srgba(css::ANTIQUE_WHITE),
            Self::SelectedEntity |
            Self::SubtractorBrush |
            Self::SelectedVertex |
            Self::SelectedPathNode |
            Self::ErrorHighlight => BevyColor::Srgba(css::RED),
            Self::NonSelectedVertex | Self::SideModeVertex => BevyColor::Srgba(css::YELLOW),
            Self::HighlightedNonSelectedEntity | Self::ToolCursor => BevyColor::Srgba(css::ORANGE),
            Self::HighlightedSelectedEntity | Self::HighlightedPath =>
            {
                BevyColor::srgb(0f32, 1f32, 0f32)
            },
            Self::ClippedPolygonsToSpawn | Self::SubtracteeBrush | Self::PathNode =>
            {
                BevyColor::Srgba(css::GOLD)
            },
            Self::OpaqueEntity => BevyColor::srgb(0.6, 0.6, 0.6),
            Self::BrushAnchor => BevyColor::srgb(0.7, 0.34, 0.05),
            Self::SpriteAnchor => BevyColor::srgb(1f32, 0.03, 0.91),
            Self::Hull => BevyColor::Srgba(css::AQUAMARINE),
            Self::CursorPolygonHull => BevyColor::srgb(0.0, 0.5, 0.0),
            Self::CursorPolygon => BevyColor::Srgba(css::AQUA),
            Self::DefaultCursor => BevyColor::Srgba(css::GRAY)
        }
    }
}

//=======================================================================//
// TYPES
//
//=======================================================================//

/// A wrapper for [`bevy::color::Color`] to format it in a specific way.
struct ColorWrapper(BevyColor);

impl std::fmt::Display for ColorWrapper
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let (r, g, b) = match_or_panic!(
            self.0,
            BevyColor::Srgba(Srgba {
                red,
                green,
                blue,
                ..
            }),
            (red, green, blue)
        );

        write!(f, "{r},{g},{b}")
    }
}

//=======================================================================//

/// The handles of the [`ColorMaterial`] associated with a [`Color`].
#[derive(Clone)]
pub(in crate::map::drawer) struct ColorHandles
{
    /// Material to draw an entity body.
    body:                 Handle<ColorMaterial>,
    /// Material to draw a line.
    line:                 Handle<ColorMaterial>,
    /// Material to draw a semitransparent line.
    semitransparent_line: Handle<ColorMaterial>
}

impl ColorHandles
{
    /// Returns a new [`ColorHandles`].
    #[inline]
    fn new(materials: &mut Assets<ColorMaterial>, color: Color, mut bevy_color: BevyColor) -> Self
    {
        const SEMITRANSPARENT_LINE_ALPHA: f32 = 1f32 / 4f32;
        const BODY_ALPHA: f32 = 1f32 / 64f32;

        if color == Color::DefaultCursor
        {
            bevy_color.set_alpha(0.5);
        }

        let mut material = ColorMaterial::from(bevy_color);
        let line = materials.add(material.clone());

        material.color.set_alpha(SEMITRANSPARENT_LINE_ALPHA);
        let semitransparent_line = materials.add(material.clone());

        material.color.set_alpha(BODY_ALPHA);
        let body = materials.add(material);

        Self {
            body,
            line,
            semitransparent_line
        }
    }
}

//=======================================================================//

/// The data associated with a [`Color`].
struct Slot
{
    /// The RGB values.
    rgb:        [f32; 3],
    /// The [`bevy::color::Color`].
    bevy_color: BevyColor,
    /// The [`egui::Color32`].
    egui_color: egui::Color32,
    /// The handles of the color materials used for the entities.
    handles:    ColorHandles
}

//=======================================================================//

/// The color settings and the related resources.
#[must_use]
pub(crate) struct ColorResources
{
    colors:      HashMap<Color, Slot>,
    solid_white: Handle<ColorMaterial>,
    solid_black: Handle<ColorMaterial>
}

impl Default for ColorResources
{
    #[inline]
    fn default() -> Self
    {
        Self {
            colors:      HashMap::with_capacity(Color::SIZE - 1),
            solid_white: Handle::default(),
            solid_black: Handle::default()
        }
    }
}

impl ColorResources
{
    /// Loads the color settings from `ini`.
    #[inline]
    pub fn load(&mut self, ini: &Ini, materials: &mut Assets<ColorMaterial>)
    {
        for color in Color::customizable_colors()
        {
            let bevy_color = match ini.get(INI_SECTION, color.config_file_key())
            {
                Some(string) =>
                {
                    /// Parses the [`bevy::color::Color`] defined in `string` and returns it. If
                    /// it cannot be parsed the default [`bevy::color::Color`] associated with
                    /// `color` is returned.
                    #[inline]
                    #[must_use]
                    fn parse(color: Color, string: &str) -> BevyColor
                    {
                        let mut vs = hv_vec![];

                        for v in string.split(',')
                        {
                            vs.push(v);
                        }

                        if vs.len() != 3
                        {
                            return color.default_bevy_color();
                        }

                        let mut rgb = [0f32; 3];

                        for (v, c) in vs.into_iter().zip(&mut rgb)
                        {
                            match v.parse::<f32>()
                            {
                                Ok(v) => *c = v.clamp(0f32, 1f32),
                                Err(_) => return color.default_bevy_color()
                            };
                        }

                        BevyColor::from_array(&rgb)
                    }

                    parse(color, &string)
                },
                None => color.default_bevy_color()
            };

            let rgb = bevy_color.to_rgb();

            self.colors.insert(color, Slot {
                rgb,
                bevy_color,
                egui_color: egui::Color32::from_array(&rgb),
                handles: ColorHandles::new(materials, color, bevy_color)
            });
        }

        self.solid_white = materials.add(ColorMaterial::from_color(BevyColor::WHITE));
        self.solid_black = materials.add(ColorMaterial::from_color(BevyColor::BLACK));
    }

    /// Returns a reference to the [`Slot`] associated with `color`.
    #[inline]
    fn get(&self, color: Color) -> &Slot { self.colors.get(&color).unwrap() }

    /// Returns a mutable reference to the [`Slot`] associated with `color`.
    #[inline]
    fn get_mut(&mut self, color: Color) -> &mut Slot { self.colors.get_mut(&color).unwrap() }

    #[inline]
    #[must_use]
    pub const fn solid_white(&self) -> &Handle<ColorMaterial> { &self.solid_white }

    #[inline]
    #[must_use]
    pub const fn solid_black(&self) -> &Handle<ColorMaterial> { &self.solid_black }

    /// Returns the [`ColorHandles`] associated with `color`.
    #[inline]
    fn handles<F: Fn(Color, &Slot) -> bool>(
        &self,
        materials: &mut Assets<ColorMaterial>,
        color: Color,
        bevy_color: BevyColor,
        f: F
    ) -> ColorHandles
    {
        self.colors
            .iter()
            .find_map(|(color, slot)| f(*color, slot).then_some(*color))
            .map_or_else(
                || ColorHandles::new(materials, color, bevy_color),
                |from_color| self.get(from_color).handles.clone()
            )
    }

    /// Returns the [`bevy::color::Color`] associated with `color`.
    #[inline]
    #[must_use]
    pub fn bevy_color(&self, color: Color) -> BevyColor { self.get(color).bevy_color }

    /// Returns the [`egui::Color32`] associated with `color`.
    #[inline]
    #[must_use]
    pub fn egui_color(&self, color: Color) -> egui::Color32
    {
        assert!(
            matches!(
                color,
                Color::PathNode |
                    Color::SelectedVertex |
                    Color::HighlightedPath |
                    Color::CursorPolygon
            ),
            "Color does not have an associated egui color."
        );

        self.get(color).egui_color
    }

    /// The associated line [`bevy::color::Color`] and draw height.
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn line_color_height(&self, color: Color) -> (BevyColor, f32)
    {
        let slot = self.get(color);
        (slot.bevy_color, color.line_height())
    }

    /// The associated brush body [`ColorMaterial`].
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn polygon_material(&self, color: Color) -> Handle<ColorMaterial>
    {
        self.get(color).handles.body.clone()
    }

    /// The associated line [`ColorMaterial`].
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn line_material(&self, color: Color) -> Handle<ColorMaterial>
    {
        self.get(color).handles.line.clone()
    }

    /// The associated semitransparent line [`ColorMaterial`].
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn semitransparent_line_material(
        &self,
        color: Color
    ) -> Handle<ColorMaterial>
    {
        self.get(color).handles.semitransparent_line.clone()
    }

    /// Saves the colors to `config`.
    #[inline]
    pub fn save(&self, config: &mut IniConfig)
    {
        for (color, slot) in &self.colors
        {
            config.set(
                INI_SECTION,
                color.config_file_key(),
                Some(format!("{}", ColorWrapper(slot.bevy_color)))
            );
        }
    }

    /// Shows the color customization options.
    #[inline]
    pub(in crate::map) fn show(&mut self, materials: &mut Assets<ColorMaterial>, ui: &mut egui::Ui)
    {
        let mut changed = None;
        let mut iter = Color::customizable_colors();

        for color in &mut iter
        {
            let slot = self.colors.get_mut(&color).unwrap();

            ui.label(color.label());
            let response = egui::color_picker::color_edit_button_rgb(ui, &mut slot.rgb);
            ui.end_row();

            if !response.changed()
            {
                continue;
            }

            changed = color.into();
            slot.bevy_color = BevyColor::from_array(&slot.rgb);
            slot.egui_color = egui::Color32::from_array(&slot.rgb);
            break;
        }

        for color in iter
        {
            ui.label(color.label());
            egui::color_picker::color_edit_button_rgb(ui, &mut self.get_mut(color).rgb);
            ui.end_row();
        }

        let changed = return_if_none!(changed);
        let bevy_color = self.get(changed).bevy_color;

        self.get_mut(changed).handles =
            self.handles(materials, changed, bevy_color, |color, slot| {
                changed != color && slot.bevy_color == bevy_color
            });
    }

    /// Resets to the default colors.
    #[inline]
    pub(in crate::map) fn reset(&mut self, materials: &mut Assets<ColorMaterial>)
    {
        for color in Color::customizable_colors()
        {
            let bevy_color = color.default_bevy_color();

            if self.get(color).bevy_color == bevy_color
            {
                continue;
            }

            let handles =
                self.handles(materials, color, bevy_color, |_, slot| slot.bevy_color == bevy_color);
            let slot = self.get_mut(color);

            slot.bevy_color = bevy_color;
            slot.rgb = slot.bevy_color.to_rgb();
            slot.egui_color = egui::Color32::from_array(&slot.rgb);
            slot.handles = handles;
        }
    }
}
