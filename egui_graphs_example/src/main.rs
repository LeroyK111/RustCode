/*
介绍egui_graphs
Egui_graphs是一个建立在流行的egui库之上的图形可视化实现。该项目即将推出第一个稳定版本，旨在通过提供易于集成、可定制的图形可视化小部件来扩展egui的可视化功能。

egui_graphs的主要特性包括：
自定义和交互性。

能够绘制任意复杂的图形与自引用，循环等。

小部件不修改所提供的图形和属性，而是与应用客户端发生交互的情况下进行更改。
*/

use egui_graphs::Elements;

pub struct ExampleApp {
    elements: Elements,
}

use eframe::CreationContext;

impl ExampleApp {
    fn new(_: &CreationContext<'_>) -> Self {
        let elements = generate_graph();
        Self { elements }
    }
}

use egui_graphs::{Edge, Elements, Node};
use std::collections::HashMap;

fn generate_graph() -> Elements {
    let mut nodes = HashMap::new();
    nodes.insert(0, Node::new(0, egui::Vec2::new(-50., 0.)));
    nodes.insert(1, Node::new(1, egui::Vec2::new(50., 0.)));
    nodes.insert(2, Node::new(2, egui::Vec2::new(-25., 60.)));
    nodes.insert(3, Node::new(3, egui::Vec2::new(25., 60.)));
    nodes.insert(4, Node::new(4, egui::Vec2::new(0., -30.)));

    let mut edges = HashMap::new();
    edges.insert((0, 1), vec![Edge::new(0, 1, 0)]);
    edges.insert((1, 2), vec![Edge::new(1, 2, 0)]);
    edges.insert((2, 4), vec![Edge::new(2, 4, 0)]);
    edges.insert((4, 3), vec![Edge::new(4, 3, 0)]);
    edges.insert((3, 0), vec![Edge::new(3, 0, 0)]);

    edges.insert((2, 0), vec![Edge::new(2, 0, 0)]);
    edges.insert((0, 4), vec![Edge::new(0, 4, 0)]);
    edges.insert((4, 1), vec![Edge::new(4, 1, 0)]);
    edges.insert((1, 3), vec![Edge::new(1, 3, 0)]);
    edges.insert((3, 2), vec![Edge::new(3, 2, 0)]);

    Elements::new(nodes, edges)
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(GraphView::new(&self.elements));
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    run_native(
        "egui_graphs_basic_demo",
        native_options,
        Box::new(|cc| Box::new(ExampleApp::new(cc))),
    )
    .unwrap();
}
