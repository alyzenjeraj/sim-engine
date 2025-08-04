use super::MapData;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const LANE_OFFSET: f32 = 0.5;
const NODE_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const ROAD_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

pub fn render_map(mut commands: Commands, map: Res<MapData>) {
    // Add a 2D camera if you don't already have one
    // commands.spawn(Camera2dBundle::default());

    // Draw nodes as small circles
    for node in &map.nodes {
        let circle = shapes::Circle {
            radius: 0.5,
            center: Vec2::ZERO,
        };

        commands.spawn((
            ShapeBuilder::with(&circle)
                .fill(Color::srgb(0.2, 0.7, 1.0)) // node color
                .build(),
            Transform::from_xyz(node.pose.x, node.pose.y, 0.0),
        ));
    }

    // Draw roads/lanes as lines
    for road in &map.roads {
        // Get the start and end node positions
        let start_node = map.nodes.iter().find(|n| n.id == road.nodes[0]).unwrap();
        let end_node = map.nodes.iter().find(|n| n.id == road.nodes[1]).unwrap();

        let start = Vec2::new(start_node.pose.x, start_node.pose.y);
        let end = Vec2::new(end_node.pose.x, end_node.pose.y);

        // Direction & normal to offset multiple lanes
        let dir = (end - start).normalize();
        let normal = Vec2::new(-dir.y, dir.x);

        for lane in &road.lanes {
            let offset = (lane.index as f32 - (road.lanes.len() as f32 - 1.0) / 2.0) * LANE_OFFSET;

            let lane_start = start + normal * offset;
            let lane_end = end + normal * offset;

            let line = shapes::Line(lane_start, lane_end);

            commands.spawn(
                ShapeBuilder::with(&line)
                    .stroke((ROAD_COLOR, 0.1)) // lane color and thickness
                    .build(),
            );
        }
    }
}
