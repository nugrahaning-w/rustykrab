#[derive(Debug, Clone)]
pub struct RustyKrabConfig {
    pub target_ios: bool,

    pub target_android: bool,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_default_config() {
        let config = RustyKrabConfig {
            target_ios: true,
            target_android: false,
        };

        assert!(config.target_ios);
    }
}
