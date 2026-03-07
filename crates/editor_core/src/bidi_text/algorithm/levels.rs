use crate::bidi_text::algorithm::types::BidiInfo;

pub fn embedding_levels(info: &BidiInfo) -> &[u8] {
    &info.levels
}
