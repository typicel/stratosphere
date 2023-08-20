#[derive(Debug)]
pub struct RecordUri {
    pub did: String,
    pub collection: String,
    pub rkey: String,
}

impl TryFrom<&str> for RecordUri {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let parts = val
            .strip_prefix("at://did:plc:")
            .ok_or(r#"record uri must start with "at://did:plc:""#)?
            .splitn(3, '/')
            .collect::<Vec<_>>();
        Ok(Self {
            did: format!("did:plc:{}", parts[0]),
            collection: parts[1].to_string(),
            rkey: parts[2].to_string(),
        })
    }
}
