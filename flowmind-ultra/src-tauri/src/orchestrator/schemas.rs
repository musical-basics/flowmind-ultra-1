use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseerOutput {
    pub sprints: Vec<SprintDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintDefinition {
    pub id: usize,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalGraph {
    pub files: Vec<GraphFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphFile {
    pub filepath: String,
    pub description: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommanderPlan {
    pub wizard_clusters: Vec<WizardCluster>,
    pub specialist_pairs: Vec<SpecialistPair>,
    pub swarm_files: Vec<SwarmFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardCluster {
    pub title: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistPair {
    pub producer_file: String,
    pub consumer_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmFile {
    pub filepath: String,
}
