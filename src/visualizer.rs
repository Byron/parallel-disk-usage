pub mod child_position;
pub mod direction;
pub mod parenthood;
pub mod proportion_bar_block;
pub mod tree;

pub use child_position::ChildPosition;
pub use direction::Direction;
pub use parenthood::Parenthood;
pub use proportion_bar_block::ProportionBarBlock;
pub use tree::{TreeHorizontalSlice, TreeSkeletalComponent, TreeSkeletalComponentVisualization};

use super::{measurement_system::MeasurementSystem, size::Size, tree::Tree};
use std::fmt::Display;

/// Visualize a [`Tree`].
#[derive(Debug)]
pub struct Visualizer<Name, Data>
where
    Name: Display,
    Data: Size,
{
    /// The tree to visualize.
    pub tree: Tree<Name, Data>,
    /// The units used to display `tree.data`.
    pub measurement_system: MeasurementSystem,
    /// The direction of the visualization of the tree.
    pub direction: Direction,
    /// Maximum number of characters/blocks can be placed in a line.
    pub max_width: u16,
}

mod methods;
