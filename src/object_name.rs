#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash)]
pub struct ObjectName {
    domain: String,
    name: String,
}

impl ObjectName {
    const NAMESPACE_OPERATOR: &str = "::";

    pub fn new(qualified_name: &str) -> Self {
        let parts = qualified_name
            .split(Self::NAMESPACE_OPERATOR)
            .collect::<Vec<&str>>();

        assert_eq!(parts.len(), 2);

        let domain = parts[0].to_string();
        let name = parts[1].to_string();

        Self { domain, name }
    }
}