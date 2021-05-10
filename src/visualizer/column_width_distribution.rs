/// Specify distribution and maximum number of characters/blocks can be placed
/// in a line.
#[derive(Debug, Clone, Copy)]
pub enum ColumnWidthDistribution {
    /// Specify maximum number of characters/blocks can be placed in a line.
    Total {
        /// Maximum number of characters/blocks can be placed in a line.
        max_width: usize,
    },
    /// Specify maximum number of characters/blocks can be placed in a line
    /// for each individual component of the visualization.
    Components {
        /// Maximum number of characters/blocks can be placed in a line
        /// for the filesystem tree visualization.
        tree_column_max_width: usize,
        /// Number of characters/blocks can be placed in a line
        /// for the proportion bar.
        bar_column_width: usize,
    },
}

pub use ColumnWidthDistribution::*;

impl ColumnWidthDistribution {
    /// Specify maximum number of characters/blocks can be placed in a line.
    #[inline]
    pub const fn total(max_width: usize) -> Self {
        Total { max_width }
    }

    /// Specify maximum number of characters/blocks can be placed in a line
    /// for each individual component of the visualization.
    #[inline]
    pub const fn components(tree_column_max_width: usize, bar_column_width: usize) -> Self {
        Components {
            tree_column_max_width,
            bar_column_width,
        }
    }

    pub(super) fn set_components(&mut self, tree_column_max_width: usize, bar_column_width: usize) {
        *self = Self::components(tree_column_max_width, bar_column_width);
    }
}
