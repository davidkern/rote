pub struct RegionPolicy {
    /// Minimum number of concurrent Regions
    min: usize,

    /// Maximum number of concurrent Regions
    max: usize,
}

impl Default for RegionPolicy {
    fn default() -> Self {
        RegionPolicy {
            min: usize::MIN,
            max: usize::MAX,
        }
    }
}

pub trait Policy {
    /// Region policy
    const REGION: RegionPolicy;
}
