use egui::{
    Color32, FontFamily, FontId, Pos2, Shape, Stroke, Vec2,
    epaint::{CircleShape, TextShape},
};
use egui_graphs::{DisplayNode, DrawContext, NodeProps};
use petgraph::Directed;

#[derive(Clone)]
pub struct TuringStateNode {
    pub pos: Pos2,

    pub selected: bool,
    pub dragged: bool,
    pub hovered: bool,
    pub inner_color: Color32,
    pub outer_color: Color32,

    pub label_text: String,

    /// Shape dependent property
    pub radius: f32,
}
impl From<NodeProps<usize>> for TuringStateNode {
    fn from(value: NodeProps<usize>) -> Self {
        return Self {
            pos: value.location(),
            selected: value.selected,
            dragged: value.dragged,
            hovered: value.hovered,
            inner_color: Color32::GRAY,
            outer_color: value.color().unwrap_or(Color32::GRAY),
            label_text: value.label.to_string(),
            radius: 5.0,
        };
    }
}
impl DisplayNode<usize, String, Directed, u32> for TuringStateNode {
    fn closest_boundary_point(&self, dir: egui::Vec2) -> egui::Pos2 {
        return Self::closest_point_on_circle(self.pos, self.radius, dir);
    }

    fn shapes(&mut self, ctx: &egui_graphs::DrawContext) -> Vec<egui::Shape> {
        let mut res: Vec<Shape> = Vec::with_capacity(2);
        let circle_center = ctx.meta.canvas_to_screen_pos(self.pos);
        let circle_radius = ctx.meta.canvas_to_screen_size(self.radius);
        let color = self.inner_color;
        let stroke = Stroke::new(1.5, Color32::WHITE);
        let galley = self.label_galley(ctx, self.radius, Color32::WHITE);
        res.push(
            CircleShape {
                center: circle_center,
                radius: circle_radius,
                fill: self.outer_color,
                stroke,
            }
            .into(),
        );
        res.push(
            CircleShape {
                center: circle_center,
                radius: circle_radius * 0.8,
                fill: self.inner_color,
                stroke,
            }
            .into(),
        );
        res.push(Self::label_shape(
            galley,
            circle_center,
            circle_radius,
            Color32::WHITE,
        ));
        return res;
    }

    fn update(&mut self, state: &egui_graphs::NodeProps<usize>) {
        self.pos = state.location();
        self.selected = state.selected;
        self.dragged = state.dragged;
        self.hovered = state.hovered;
        self.label_text = state.label.to_string();
        self.outer_color = state.color().unwrap_or(Color32::GRAY);
        self.inner_color = match self.selected {
            true => Color32::from_rgb(54, 172, 21),
            false => Color32::from_rgb(33, 33, 33),
        };
    }

    fn is_inside(&self, pos: egui::Pos2) -> bool {
        return Self::is_inside_circle(self.pos, self.radius, pos);
    }
}
impl TuringStateNode {
    fn is_inside_circle(center: Pos2, radius: f32, pos: Pos2) -> bool {
        let dir = pos - center;
        dir.length() <= radius
    }
    fn closest_point_on_circle(center: Pos2, radius: f32, dir: Vec2) -> Pos2 {
        center + dir.normalized() * radius
    }
    fn label_galley(
        &self,
        ctx: &DrawContext,
        radius: f32,
        color: Color32,
    ) -> std::sync::Arc<egui::Galley> {
        ctx.ctx.fonts_mut(|f| {
            f.layout_no_wrap(
                self.label_text.clone(),
                FontId::new(radius * 10.0, FontFamily::Monospace),
                color,
            )
        })
    }
    fn label_shape(
        galley: std::sync::Arc<egui::Galley>,
        center: Pos2,
        _radius: f32,
        color: Color32,
    ) -> Shape {
        let label_pos = Pos2::new(
            center.x - galley.size().x / 2.0,
            center.y - galley.size().y / 2.0,
        );
        TextShape::new(label_pos, galley, color).into()
    }
}
