//=======================================================================//
// IMPORTS
//
//=======================================================================//

use super::{Animation, AtlasAnimation, Timing};
use crate::{
    map::{hv_vec, HvVec},
    utils::overall_value::{OverallValue, OverallValueInterface, UiOverallValue}
};

//=======================================================================//
// ENUMS
//
//=======================================================================//

#[must_use]
#[derive(Default, Debug)]
pub(in crate::map) enum OverallTiming
{
    #[default]
    None,
    NonUniform,
    Uniform(OverallValue<f32>),
    PerFrame(OverallValue<Vec<f32>>)
}

impl From<&Timing> for OverallTiming
{
    #[inline]
    fn from(value: &Timing) -> Self
    {
        match value
        {
            Timing::Uniform(value) => Self::Uniform((*value).into()),
            Timing::PerFrame(vec) => Self::PerFrame(vec.clone().into())
        }
    }
}

impl OverallValueInterface<Timing> for OverallTiming
{
    #[inline]
    fn is_not_uniform(&self) -> bool { matches!(self, Self::NonUniform) }

    #[inline]
    fn stack(&mut self, value: &Timing) -> bool
    {
        match (&mut *self, value)
        {
            (Self::None, _) => *self = value.into(),
            (Self::Uniform(v_0), Timing::Uniform(v_1)) => _ = v_0.stack(v_1),
            (Self::Uniform(_), Timing::PerFrame(_)) | (Self::PerFrame(_), Timing::Uniform(_)) =>
            {
                *self = Self::NonUniform;
            },
            (Self::PerFrame(vec_0), Timing::PerFrame(vec_1)) =>
            {
                _ = vec_0.stack(vec_1);
            },
            _ => ()
        };

        self.is_not_uniform()
    }

    #[inline]
    fn merge(&mut self, other: Self) -> bool
    {
        if let Self::None = self
        {
            *self = other;
            return self.is_not_uniform();
        }

        match (&mut *self, other)
        {
            (_, Self::None) | (Self::NonUniform, _) => (),
            (_, OverallTiming::NonUniform) |
            (OverallTiming::Uniform(_), OverallTiming::PerFrame(_)) |
            (OverallTiming::PerFrame(_), OverallTiming::Uniform(_)) => *self = Self::NonUniform,
            (OverallTiming::Uniform(v_0), OverallTiming::Uniform(v_1)) => _ = v_0.merge(v_1),
            (OverallTiming::PerFrame(vec_0), OverallTiming::PerFrame(vec_1)) =>
            {
                _ = vec_0.merge(vec_1);
            },
            (Self::None, _) => unreachable!()
        };

        self.is_not_uniform()
    }
}

//=======================================================================//

#[must_use]
pub(in crate::map) enum UiOverallTiming
{
    None,
    NonUniform,
    Single(UiOverallValue<f32>),
    PerFrame(HvVec<UiOverallValue<f32>>)
}

impl From<OverallTiming> for UiOverallTiming
{
    #[inline]
    fn from(value: OverallTiming) -> Self
    {
        match value
        {
            OverallTiming::None => Self::None,
            OverallTiming::NonUniform => Self::NonUniform,
            OverallTiming::Uniform(value) => Self::Single(value.into()),
            OverallTiming::PerFrame(vec) =>
            {
                match vec
                {
                    OverallValue::None => unreachable!(),
                    OverallValue::NonUniform =>
                    {
                        Self::PerFrame(hv_vec![UiOverallValue::non_uniform()])
                    },
                    OverallValue::Uniform(vec) =>
                    {
                        Self::PerFrame(
                            hv_vec![collect; vec.into_iter().map(std::convert::Into::into)]
                        )
                    },
                }
            },
        }
    }
}

//=======================================================================//

#[must_use]
#[derive(Default, Debug)]
pub(in crate::map) enum OverallAnimation
{
    #[default]
    NoSelection,
    None,
    NonUniform,
    List(OverallValue<Vec<(String, f32)>>),
    Atlas(OverallAtlasAnimation)
}

impl From<&Animation> for OverallAnimation
{
    #[inline]
    fn from(value: &Animation) -> Self
    {
        match value
        {
            Animation::None => Self::None,
            Animation::List(anim) => Self::List(anim.0.clone().into()),
            Animation::Atlas(anim) => Self::Atlas(anim.into())
        }
    }
}

impl OverallValueInterface<Animation> for OverallAnimation
{
    #[inline]
    fn is_not_uniform(&self) -> bool { matches!(self, Self::NonUniform) }

    #[inline]
    fn stack(&mut self, value: &Animation) -> bool
    {
        match (&mut *self, value)
        {
            (Self::NoSelection, _) => *self = value.into(),
            (Self::NonUniform, _) | (Self::None, Animation::None) => (),
            (Self::List(vec), Animation::List(anim)) => _ = vec.stack(&anim.0),
            (Self::Atlas(atlas), Animation::Atlas(anim)) =>
            {
                _ = atlas.x.stack(&anim.x);
                _ = atlas.y.stack(&anim.y);
                _ = atlas.len.stack(&anim.len);
                _ = atlas.timing.stack(&anim.timing);
            },
            _ => *self = Self::NonUniform
        };

        self.is_not_uniform()
    }

    #[inline]
    fn merge(&mut self, other: Self) -> bool
    {
        if let Self::NoSelection = self
        {
            *self = other;
            return self.is_not_uniform();
        }

        match (&mut *self, other)
        {
            (Self::NoSelection, _) => unreachable!(),
            (Self::NonUniform, _) | (_, Self::NoSelection) | (Self::None, Self::None) => (),
            (Self::List(v_0), Self::List(v_1)) =>
            {
                match (&mut *v_0, v_1)
                {
                    (OverallValue::None, _) | (_, OverallValue::None) => unreachable!(),
                    (OverallValue::NonUniform, _) | (_, OverallValue::NonUniform) =>
                    {
                        *v_0 = OverallValue::NonUniform;
                    },
                    (OverallValue::Uniform(vec_0), OverallValue::Uniform(vec_1)) =>
                    {
                        if *vec_0 != *vec_1
                        {
                            *v_0 = OverallValue::NonUniform;
                        }
                    }
                };
            },
            (Self::Atlas(atlas_0), Self::Atlas(atlas_1)) =>
            {
                _ = atlas_0.x.merge(atlas_1.x);
                _ = atlas_0.y.merge(atlas_1.y);
                _ = atlas_0.len.merge(atlas_1.len);
                _ = atlas_0.timing.merge(atlas_1.timing);
            },
            _ => *self = Self::NonUniform
        }

        self.is_not_uniform()
    }
}

//=======================================================================//

#[must_use]
#[derive(Debug)]
pub(in crate::map) struct OverallAtlasAnimation
{
    pub x:      OverallValue<u32>,
    pub y:      OverallValue<u32>,
    pub len:    OverallValue<usize>,
    pub timing: OverallTiming
}

impl From<&AtlasAnimation> for OverallAtlasAnimation
{
    #[inline]
    fn from(value: &AtlasAnimation) -> Self
    {
        Self {
            x:      value.x.into(),
            y:      value.y.into(),
            len:    value.len.into(),
            timing: (&value.timing).into()
        }
    }
}

//=======================================================================//

#[must_use]
pub(in crate::map) enum UiOverallListAnimation
{
    NonUniform(UiOverallValue<String>),
    Uniform(HvVec<(UiOverallValue<String>, UiOverallValue<f32>)>, UiOverallValue<String>)
}

impl From<OverallValue<Vec<(String, f32)>>> for UiOverallListAnimation
{
    #[inline]
    fn from(value: OverallValue<Vec<(String, f32)>>) -> Self
    {
        match value
        {
            OverallValue::None => unreachable!(),
            OverallValue::NonUniform => Self::NonUniform(UiOverallValue::non_uniform()),
            OverallValue::Uniform(vec) =>
            {
                Self::Uniform(
                    hv_vec![collect; vec.into_iter().map(|(name, time)| {
                        (name.into(), time.into())
                    })],
                    UiOverallValue::non_uniform()
                )
            },
        }
    }
}

//=======================================================================//

#[must_use]
pub(in crate::map) struct UiOverallAtlasAnimation
{
    pub x:      UiOverallValue<u32>,
    pub y:      UiOverallValue<u32>,
    pub len:    UiOverallValue<usize>,
    pub timing: UiOverallTiming
}

impl From<OverallAtlasAnimation> for UiOverallAtlasAnimation
{
    #[inline]
    fn from(value: OverallAtlasAnimation) -> Self
    {
        Self {
            x:      value.x.into(),
            y:      value.y.into(),
            len:    value.len.into(),
            timing: value.timing.into()
        }
    }
}

//=======================================================================//

#[must_use]
#[derive(Default)]
pub(in crate::map) enum UiOverallAnimation
{
    #[default]
    NoSelection,
    NonUniform,
    None,
    List(UiOverallListAnimation),
    Atlas(UiOverallAtlasAnimation)
}

impl From<OverallAnimation> for UiOverallAnimation
{
    #[inline]
    fn from(value: OverallAnimation) -> Self
    {
        match value
        {
            OverallAnimation::NoSelection => Self::NoSelection,
            OverallAnimation::NonUniform => Self::NonUniform,
            OverallAnimation::None => Self::None,
            OverallAnimation::List(list) => Self::List(list.into()),
            OverallAnimation::Atlas(atlas) => Self::Atlas(atlas.into())
        }
    }
}
