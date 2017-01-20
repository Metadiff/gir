use primitives::*;
use std::path::PathBuf;

/// Collection of policies used in the graph
#[derive(Clone, Copy, Debug)]
pub struct GraphPolicies {
    pub implicit_broadcast: Policy,
    pub implicit_cast: Policy,
    pub independent_derivative: Policy
}

impl Default for GraphPolicies {
    fn default() -> Self {
        GraphPolicies{
            implicit_broadcast: Policy::Warn,
            implicit_cast: Policy::Warn,
            independent_derivative: Policy::Warn
        }
    }
}

pub trait LoadFromSystem: Default {
    fn load_from_system() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug)]
pub struct GraphProperties {
    pub http_proxy: Option<String>,
    pub scope_delimiter: String,
    pub policies: GraphPolicies,
    pub default_work_dir: PathBuf
}

impl Default for GraphProperties {
    fn default() -> Self {
        GraphProperties {
            http_proxy: None,
            scope_delimiter: "::".into(),
            policies: GraphPolicies::default(),
            default_work_dir: PathBuf::from("~/.gir")
        }
    }
}

impl LoadFromSystem for GraphProperties {}


