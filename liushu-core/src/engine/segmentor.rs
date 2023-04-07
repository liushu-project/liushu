use patricia_tree::PatriciaMap;

pub fn split_pinyin(code: &str, trie: &PatriciaMap<Vec<String>>) -> Vec<String> {
    let mut syllables = Vec::new();
    let mut remaining = code;
    while !remaining.is_empty() {
        if let Some((bytes, _)) = trie.get_longest_common_prefix(remaining) {
            let match_str = String::from_utf8_lossy(bytes);
            let match_str = match_str.trim();
            syllables.push(match_str.to_string());
            remaining = &remaining[match_str.len()..];
        } else {
            break;
        }
    }
    syllables
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_pinyin() {
        let mut trie = PatriciaMap::new();
        trie.insert_str("nihao", vec!["你好".to_string()]);
        trie.insert_str("ke", vec!["可".to_string()]);
        trie.insert_str("yi", vec!["以".to_string()]);
        trie.insert_str("a", vec!["啊".to_string()]);

        assert_eq!(split_pinyin("nihaoa", &trie), vec!["nihao", "a"]);
        assert_eq!(split_pinyin("keyi", &trie), vec!["ke", "yi"]);
        assert_eq!(split_pinyin("aaaaa", &trie), vec!["a", "a", "a", "a", "a"]);

        assert_eq!(split_pinyin("keyide", &trie), vec!["ke", "yi"]);
        assert_eq!(split_pinyin("", &trie), vec![] as Vec<String>);
    }
}
