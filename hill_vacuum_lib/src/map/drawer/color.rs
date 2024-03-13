//=======================================================================//
// IMPORTS
//
//=======================================================================//

use bevy::prelude::*;
use bevy_egui::egui;
use proc_macros::{color_height, EnumFromUsize, EnumIter, EnumSize};

use crate::map::EGUI_CYAN;

//=======================================================================//
// ENUMS
//
//=======================================================================//

/// The colors used by the map editor.
#[derive(Copy, Clone, Debug, EnumIter, EnumSize, EnumFromUsize)]
pub(in crate::map) enum Color
{
    /// The background color.
    Clear,
    /// The color of the softer colored grids.
    SoftGridLines,
    /// The color of the grid lines.
    GridLines,
    /// The color of the lines representing the x and y axis.
    OriginGridLines,
    /// The color of the selected brush hull lines extensions.
    HullExtensions,
    /// The color of the brushes that are not selected.
    NonSelectedBrush,
    /// The color of the selected brushes.
    SelectedBrush,
    /// The color of the non selected vertexes.
    NonSelectedVertex,
    /// The color of the selected vertexes.
    SelectedVertex,
    /// The color of the brush sides while using the side tool.
    SideModeVertex,
    /// The color of the brushes to which the subtraction is being applied.
    SubtracteeBrush,
    /// The color the highlighted non selected brush.
    HighlightedNonSelectedBrush,
    /// The color of the highlighted selected brush.
    HighlightedSelectedBrush,
    /// The color of the brushes generated by the clip tool that will be spawned in the map.
    ClippedPolygonsToSpawn,
    /// The color of the brushes generated by the clip tool that will not be spawned in the map.
    ClippedPolygonsNotToSpawn,
    /// The color of the brush that will be subtracted from the other selected brushes.
    SubtractorBrush,
    /// The color of brushes that are not relevant to the purposes of the tool being used.
    OpaqueBrush,
    /// The color of the path tool path nodes.
    PathNode,
    /// The color of the lines showing the brushes tied together.
    BrushAnchor,
    /// The color of the lines showing the brushes tied together.
    SpriteAnchor,
    /// The color of the selected path tool path node.
    SelectedPathNode,
    /// The color of the path tool path node candidate.
    PathNodeCandidate,
    HighlightedPath,
    /// The color of the [`Hull`]s' outlines.
    Hull,
    /// The color of the cursor.
    DefaultCursor,
    /// The color of the [`Hull`] of the cursor cursor polygons of the draw tools.
    CursorPolygonHull,
    /// The generic color used for the cursor by some tools.
    ToolCursor,
    /// The color of the cursor polygons of the draw tools.
    CursorPolygon,
    ErrorHighlight
}

impl Color
{
    color_height!(
        Clear,
        SoftGridLines,
        GridLines,
        OriginGridLines,
        HullExtensions,
        ClippedPolygonsNotToSpawn | OpaqueBrush,
        NonSelectedBrush,
        SelectedBrush,
        SubtracteeBrush,
        ClippedPolygonsToSpawn,
        HighlightedSelectedBrush | HighlightedNonSelectedBrush,
        NonSelectedVertex,
        SelectedVertex,
        SideModeVertex,
        BrushAnchor,
        SpriteAnchor,
        PathNode,
        HighlightedPath,
        SelectedPathNode | PathNodeCandidate,
        SubtractorBrush,
        Hull,
        CursorPolygonHull,
        DefaultCursor,
        ToolCursor | CursorPolygon,
        ErrorHighlight
    );

    /// The [`bevy::prelude::Color`] which corresponds to [`Color`].
    #[inline]
    #[must_use]
    pub const fn bevy_color(self) -> bevy::prelude::Color
    {
        use bevy::prelude::Color;

        match self
        {
            Self::Clear => Color::BLACK,
            Self::SoftGridLines => Color::rgb(0.04, 0.06, 0.09),
            Self::GridLines | Self::ClippedPolygonsNotToSpawn => Color::DARK_GRAY,
            Self::OriginGridLines => Color::WHITE,
            Self::HullExtensions => Color::INDIGO,
            Self::NonSelectedBrush => Color::ANTIQUE_WHITE,
            Self::SelectedBrush |
            Self::SubtractorBrush |
            Self::SelectedVertex |
            Self::SelectedPathNode |
            Self::PathNodeCandidate |
            Self::ErrorHighlight => Color::RED,
            Self::NonSelectedVertex | Self::SideModeVertex => Color::YELLOW,
            Self::HighlightedNonSelectedBrush | Self::ToolCursor => Color::ORANGE,
            Self::HighlightedSelectedBrush | Self::HighlightedPath => Color::GREEN,
            Self::ClippedPolygonsToSpawn | Self::SubtracteeBrush | Self::PathNode => Color::GOLD,
            Self::OpaqueBrush => Color::rgb(0.6, 0.6, 0.6),
            Self::BrushAnchor => Color::rgb(0.7, 0.34, 0.05),
            Self::SpriteAnchor => Color::rgb(1f32, 0.03, 0.91),
            Self::Hull => Color::AQUAMARINE,
            Self::CursorPolygonHull => Color::DARK_GREEN,
            Self::CursorPolygon => Color::CYAN,
            Self::DefaultCursor => Color::GRAY
        }
    }

    #[inline]
    #[must_use]
    pub const fn egui_color(self) -> egui::Color32
    {
        match self
        {
            Self::PathNode => egui::Color32::GOLD,
            Self::PathNodeCandidate | Self::SelectedVertex => egui::Color32::RED,
            Self::HighlightedPath => egui::Color32::GREEN,
            Self::CursorPolygon => EGUI_CYAN,
            _ => unreachable!()
        }
    }

    /// The [`ColorMaterials`] associated with the [`Color`] values.
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn materials(materials: &mut Assets<ColorMaterial>)
        -> ColorMaterials
    {
        std::array::from_fn(|i| {
            let mut material = ColorMaterial::from(Color::from(i).bevy_color());

            if i == Color::DefaultCursor as usize
            {
                material.color.set_a(0.5);
            }

            let line_color = materials.add(material.clone());

            material.color.set_a(0.25);
            let st_line_color = materials.add(material.clone());

            material.color.set_a(0.021_875);
            let body_color = materials.add(material);

            ColorHandles {
                body:                 body_color,
                line:                 line_color,
                semitransparent_line: st_line_color
            }
        })
    }

    /// The associated brush body [`ColorMaterial`].
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn brush_material(
        self,
        materials: &ColorMaterials
    ) -> Handle<ColorMaterial>
    {
        materials[self as usize].body.clone()
    }

    /// The associated line [`ColorMaterial`].
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn line_material(
        self,
        materials: &ColorMaterials
    ) -> Handle<ColorMaterial>
    {
        materials[self as usize].line.clone()
    }

    /// The associated semitransparent line [`ColorMaterial`].
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn semitransparent_line_material(
        self,
        materials: &ColorMaterials
    ) -> Handle<ColorMaterial>
    {
        materials[self as usize].semitransparent_line.clone()
    }

    /// The associated line [`bevy::prelude::Color`] and draw height.
    #[inline]
    #[must_use]
    pub(in crate::map::drawer) fn line_color_height(self) -> (bevy::prelude::Color, f32)
    {
        (self.bevy_color(), self.line_height())
    }
}

//=======================================================================//
// TYPES
//
//=======================================================================//

/// The handles of all the [`ColorMaterial`]s used in the editor.
pub(in crate::map::drawer) type ColorMaterials = [ColorHandles; Color::SIZE];

//=======================================================================//

/// The handles of the [`ColorMaterial`] associated with a [`Color`].
pub(in crate::map::drawer) struct ColorHandles
{
    body:                 Handle<ColorMaterial>,
    line:                 Handle<ColorMaterial>,
    semitransparent_line: Handle<ColorMaterial>
}

impl Default for ColorHandles
{
    #[inline]
    fn default() -> Self
    {
        Self {
            body:                 Handle::default(),
            line:                 Handle::default(),
            semitransparent_line: Handle::default()
        }
    }
}
