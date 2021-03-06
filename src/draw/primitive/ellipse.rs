use crate::color::conv::IntoLinSrgba;
use crate::draw::primitive::polygon::{
    PolygonIndices, PolygonInit, PolygonOptions, PolygonVertices, SetPolygon,
};
use crate::draw::primitive::Primitive;
use crate::draw::properties::spatial::{dimension, orientation, position};
use crate::draw::properties::{
    spatial, ColorScalar, Draw, Drawn, IntoDrawn, LinSrgba, SetColor, SetDimensions,
    SetOrientation, SetPosition, SetStroke,
};
use crate::draw::{theme, Drawing};
use crate::geom::{self, Vector2};
use crate::math::BaseFloat;
use lyon::tessellation::StrokeOptions;

/// Properties related to drawing an **Ellipse**.
#[derive(Clone, Debug)]
pub struct Ellipse<S = geom::scalar::Default> {
    dimensions: spatial::dimension::Properties<S>,
    resolution: Option<usize>,
    polygon: PolygonInit<S>,
}

/// The drawing context for an ellipse.
pub type DrawingEllipse<'a, S = geom::scalar::Default> = Drawing<'a, Ellipse<S>, S>;

// Ellipse-specific methods.

impl<S> Ellipse<S>
where
    S: BaseFloat,
{
    /// Stroke the outline with the given color.
    pub fn stroke<C>(self, color: C) -> Self
    where
        C: IntoLinSrgba<ColorScalar>,
    {
        self.stroke_color(color)
    }

    /// Specify the width and height of the **Ellipse** via a given **radius**.
    pub fn radius(self, radius: S) -> Self {
        let side = radius * (S::one() + S::one());
        self.w_h(side, side)
    }

    /// The number of sides used to draw the ellipse.
    pub fn resolution(mut self, resolution: usize) -> Self {
        self.resolution = Some(resolution);
        self
    }
}

// Trait implementations.

impl<S> IntoDrawn<S> for Ellipse<S>
where
    S: BaseFloat,
{
    type Vertices = PolygonVertices;
    type Indices = PolygonIndices;
    fn into_drawn(self, mut draw: Draw<S>) -> Drawn<S, Self::Vertices, Self::Indices> {
        let Ellipse {
            dimensions,
            polygon,
            resolution,
        } = self;

        // First get the dimensions of the ellipse.
        let (maybe_x, maybe_y, maybe_z) = dimensions.to_scalars(&draw);
        assert!(
            maybe_z.is_none(),
            "z dimension support for ellipse is unimplemented"
        );

        // TODO: These should probably be adjustable via Theme.
        const DEFAULT_RESOLUTION: usize = 50;
        let default_w = || S::from(100.0).unwrap();
        let default_h = || S::from(100.0).unwrap();
        let w = maybe_x.unwrap_or_else(default_w);
        let h = maybe_y.unwrap_or_else(default_h);
        let resolution = resolution.unwrap_or(DEFAULT_RESOLUTION);
        let rect = geom::Rect::from_wh(Vector2 { x: w, y: h });
        let ellipse = geom::Ellipse::new(rect, resolution);
        let points = ellipse.circumference();
        let polygon = draw.drawing_context(|ctxt| polygon.points(ctxt, points));
        polygon.into_drawn_themed(draw, &theme::Primitive::Ellipse)
    }
}

impl<S> Default for Ellipse<S> {
    fn default() -> Self {
        let dimensions = Default::default();
        let polygon = Default::default();
        let resolution = Default::default();
        Ellipse {
            dimensions,
            polygon,
            resolution,
        }
    }
}

impl<S> SetOrientation<S> for Ellipse<S> {
    fn properties(&mut self) -> &mut orientation::Properties<S> {
        SetOrientation::properties(&mut self.polygon)
    }
}

impl<S> SetPosition<S> for Ellipse<S> {
    fn properties(&mut self) -> &mut position::Properties<S> {
        SetPosition::properties(&mut self.polygon)
    }
}

impl<S> SetDimensions<S> for Ellipse<S> {
    fn properties(&mut self) -> &mut dimension::Properties<S> {
        SetDimensions::properties(&mut self.dimensions)
    }
}

impl<S> SetColor<ColorScalar> for Ellipse<S> {
    fn rgba_mut(&mut self) -> &mut Option<LinSrgba> {
        SetColor::rgba_mut(&mut self.polygon)
    }
}

impl<S> SetStroke for Ellipse<S> {
    fn stroke_options_mut(&mut self) -> &mut StrokeOptions {
        SetStroke::stroke_options_mut(&mut self.polygon)
    }
}

impl<S> SetPolygon<S> for Ellipse<S> {
    fn polygon_options_mut(&mut self) -> &mut PolygonOptions<S> {
        SetPolygon::polygon_options_mut(&mut self.polygon)
    }
}

// Primitive conversion.

impl<S> From<Ellipse<S>> for Primitive<S> {
    fn from(prim: Ellipse<S>) -> Self {
        Primitive::Ellipse(prim)
    }
}

impl<S> Into<Option<Ellipse<S>>> for Primitive<S> {
    fn into(self) -> Option<Ellipse<S>> {
        match self {
            Primitive::Ellipse(prim) => Some(prim),
            _ => None,
        }
    }
}

// Drawing methods.

impl<'a, S> DrawingEllipse<'a, S>
where
    S: BaseFloat,
{
    /// Stroke the outline with the given color.
    pub fn stroke<C>(self, color: C) -> Self
    where
        C: IntoLinSrgba<ColorScalar>,
    {
        self.map_ty(|ty| ty.stroke(color))
    }

    /// Specify the width and height of the **Ellipse** via a given **radius**.
    pub fn radius(self, radius: S) -> Self {
        self.map_ty(|ty| ty.radius(radius))
    }

    /// The number of sides used to draw the ellipse.
    pub fn resolution(self, resolution: usize) -> Self {
        self.map_ty(|ty| ty.resolution(resolution))
    }
}
