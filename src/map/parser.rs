use bevy::prelude::*;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

#[derive(Deserialize, Debug)]
pub struct Node {
    pub id: u32,
    pub pose: Pose,
    pub variant: String,
}

#[derive(Debug, Deserialize)]
pub struct Lane {
    pub id: String,
    pub index: u32,
    pub direction: String,
    pub speed_limit: f32,
}

#[derive(Debug, Deserialize)]
pub struct Road {
    pub id: u32,
    pub nodes: [u32; 2],
    pub bidirectional: bool,
    pub lanes: Vec<Lane>,
}

#[derive(Debug, Deserialize, Resource)]
pub struct MapData {
    pub map_name: String,
    pub nodes: Vec<Node>,
    pub roads: Vec<Road>,
}

impl MapData {
    pub fn parse_file(path: &str) -> Self {
        let file_path = format!("assets/maps/{}", path);
        let file = File::open(file_path).expect("Failed to open map file");
        serde_json::from_reader(file).expect("Failed to parse map file")
    }
}
