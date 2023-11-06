use patricia_tree::PatriciaMap;

pub trait Segmentor {
    fn segment(&self, code: &str) -> Vec<String>;
}

impl<V> Segmentor for PatriciaMap<V> {
    fn segment(&self, code: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut remaining = code;
        while !remaining.is_empty() {
            if let Some((bytes, _)) = self.get_longest_common_prefix(remaining) {
                let match_str = String::from_utf8_lossy(bytes);
                let match_str = match_str.trim();
                result.push(match_str.to_string());
                remaining = &remaining[match_str.len()..];
            } else {
                break;
            }
        }
        result
    }
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

        assert_eq!(trie.segment("nihaoa"), vec!["nihao", "a"]);
        assert_eq!(trie.segment("keyi"), vec!["ke", "yi"]);
        assert_eq!(trie.segment("aaaaa"), vec!["a", "a", "a", "a", "a"]);

        assert_eq!(trie.segment("keyide"), vec!["ke", "yi"]);
        assert_eq!(trie.segment(""), vec![] as Vec<String>);
    }
}
