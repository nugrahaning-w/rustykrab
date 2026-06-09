/// Represents the semantic type of an AST node.
///
/// NodeKind determines:
/// - Validation rules
/// - Generator behavior
/// - Tree structure constraints
///
/// The AST remains platform-independent.
/// Generators map NodeKind to platform-specific widgets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    // Basic Widgets
    Text,
    Button,
    Image,
    TextField,

    // Layout Widgets
    VStack,
    HStack,
    ZStack,
    ScrollView,

    Spacer,

    // Navigation Widgets
    NavigationView,
    NavigationLink,

    // Collection Widgets
    List,
    Grid,

    // Plugin / Third-party Widgets
    Custom(String),
}

impl NodeKind {
    /// Returns true if this node can contain children.
    pub fn is_container(&self) -> bool {
        matches!(
            self,
            Self::VStack
                | Self::HStack
                | Self::ZStack
                | Self::ScrollView
                | Self::NavigationView
                | Self::NavigationLink
                | Self::List
                | Self::Grid
        )
    }

    /// Returns true if this node is a leaf node.
    pub fn is_leaf(&self) -> bool {
        !self.is_container()
    }

    /// Returns the display name of the node kind.
    pub fn name(&self) -> &str {
        match self {
            Self::Text => "Text",
            Self::Button => "Button",
            Self::Image => "Image",
            Self::TextField => "TextField",

            Self::VStack => "VStack",
            Self::HStack => "HStack",
            Self::ZStack => "ZStack",
            Self::ScrollView => "ScrollView",

            Self::Spacer => "Spacer",

            Self::NavigationView => "NavigationView",
            Self::NavigationLink => "NavigationLink",

            Self::List => "List",
            Self::Grid => "Grid",

            Self::Custom(name) => name,
        }
    }
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_leaf() {
        assert!(NodeKind::Text.is_leaf());
        assert!(!NodeKind::Text.is_container());
    }

    #[test]
    fn button_is_leaf() {
        assert!(NodeKind::Button.is_leaf());
        assert!(!NodeKind::Button.is_container());
    }

    #[test]
    fn vstack_is_container() {
        assert!(NodeKind::VStack.is_container());
        assert!(!NodeKind::VStack.is_leaf());
    }

    #[test]
    fn hstack_is_container() {
        assert!(NodeKind::HStack.is_container());
    }

    #[test]
    fn navigation_view_is_container() {
        assert!(NodeKind::NavigationView.is_container());
    }

    #[test]
    fn custom_node_name() {
        let kind = NodeKind::Custom("ChartView".into());

        assert_eq!(kind.name(), "ChartView");
    }

    #[test]
    fn display_node_kind() {
        assert_eq!(NodeKind::Text.to_string(), "Text");
    }
}
