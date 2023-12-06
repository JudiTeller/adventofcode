
#[derive(Debug, Copy, Clone)]
pub struct SeedMapping {
    pub idx: [i64; 8],
}


#[derive(Debug, Copy, Clone)]
pub struct SeedMappingRanged {
    pub start: i64,
    pub range: i64,
    pub idx: [i64; 8],
}