use egui::{Color32, Pos2};
use egui_graphs::DisplayNode;
use petgraph::Directed;

pub struct TuringStateNode {
    radius: f32,
}
impl DisplayNode<usize, String, Directed, u32> for TuringStateNode {
    fn closest_boundary_point(&self, dir: egui::Vec2) -> egui::Pos2 {
        return Pos2 {
            x: dir[0] * self.radius,
            y: dir[1] * self.radius,
        };
    }

    fn shapes(&mut self, ctx: &egui_graphs::DrawContext) -> Vec<egui::Shape> {
        todo!();
    }

    fn update(&mut self, state: &egui_graphs::NodeProps<usize>) {
        todo!()
    }

    fn is_inside(&self, pos: egui::Pos2) -> bool {
        todo!()
    }
}
pub struct TuringTransitionEdge;
