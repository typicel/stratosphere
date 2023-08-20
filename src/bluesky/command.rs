use std::path::PathBuf;

#[derive(Debug)]
pub struct CreateRecordPostArgs {
    pub text: String,
    // reply: Option<String>,
    // image: Option<Vec<PathBuf>>,
}

#[derive(Debug)]
pub struct CreateRecordLikeArgs {
    pub uri: String,
}

#[derive(Debug)]
pub enum CreateRecordCommand {
    Post(CreateRecordPostArgs),
    // Repost(CreateRecordRepostArgs),
    Like(CreateRecordLikeArgs),
    // Block(CreateRecordBlockArgs),
}

#[derive(Debug)]
pub enum Command {
    CreateRecord(CreateRecordCommand),
    GetTimeline,
}
