use patricia_tree::StringPatriciaMap;

pub trait Segmentor {
    fn segment(&self, code: &str) -> Vec<String>;
}

impl<V> Segmentor for StringPatriciaMap<V> {
    fn segment(&self, code: &str) -> Vec<String> {
        let mut result = vec![];
        let mut last = 0;
        while last < code.len() {
            let lcp = self.longest_common_prefix_len(&code[last..]);
            if lcp > 0 {
                result.push(code[last..last + lcp].to_string());
                last += lcp;
            } else {
                result.push(code[last..].to_string());
                last = code.len();
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
        let mut trie = StringPatriciaMap::new();
        trie.insert("nihao", vec!["你好".to_string()]);
        trie.insert("ke", vec!["可".to_string()]);
        trie.insert("yi", vec!["以".to_string()]);
        trie.insert("a", vec!["啊".to_string()]);

        assert_eq!(trie.segment("nihaoa"), vec!["nihao", "a"]);
        assert_eq!(trie.segment("keyi"), vec!["ke", "yi"]);
        assert_eq!(trie.segment("aaaaa"), vec!["a", "a", "a", "a", "a"]);

        // empty
        assert_eq!(trie.segment(""), vec![] as Vec<String>);

        // unrecognized
        assert_eq!(trie.segment("keyide"), vec!["ke", "yi", "de"]);
        assert_eq!(trie.segment("ni"), vec!["ni"]);

        // partial
        assert_eq!(trie.segment("keya"), vec!["ke", "y", "a"]);
        assert_eq!(trie.segment("nihke"), vec!["nih", "ke"]);
    }
}
