pub use pinyin::ToPinyin;

pub const POSIBLE_PINYINS: [&str; 681] = [
    "jang", "chv", "gen", "tui", "wou", "her", "xao", "xan", "zuan", "ca", "bi", "pun", "co",
    "shui", "kong", "si", "chve", "hin", "che", "xing", "kan", "suan", "an", "kuang", "su", "se",
    "der", "biu", "jiu", "he", "ne", "buan", "jua", "ya", "gong", "yve", "zia", "bua", "chun",
    "lui", "o", "sua", "ber", "yen", "wa", "ta", "fo", "kun", "hang", "xia", "fun", "lai", "lu",
    "dan", "yai", "je", "pie", "zhong", "puai", "juo", "za", "nun", "chong", "yuo", "feng", "lv",
    "kin", "xua", "cha", "gou", "mi", "cen", "dv", "nve", "keng", "buai", "zen", "nan", "dong",
    "zhe", "cuai", "ren", "tua", "ser", "tun", "zhai", "kui", "mun", "man", "guo", "wao", "dou",
    "lua", "guan", "hu", "duai", "kie", "fi", "bao", "beng", "cve", "ruo", "hai", "duan", "hua",
    "hiu", "sia", "wia", "win", "qv", "fie", "hve", "puang", "gu", "dei", "qeng", "cv", "fuo",
    "zhen", "sa", "bai", "lou", "muang", "wei", "pan", "lang", "ti", "cer", "weng", "qa", "yv",
    "a", "wen", "kang", "shan", "shei", "cie", "luang", "zuai", "gia", "fing", "bv", "sheng", "ji",
    "wui", "zer", "meng", "ger", "fv", "mv", "duo", "tai", "ce", "kiu", "dao", "jong", "so", "pv",
    "lia", "nv", "mu", "ruan", "cuang", "tan", "yia", "ba", "pin", "po", "nuai", "yan", "nuo",
    "yuai", "qao", "cia", "sui", "leng", "fve", "bang", "pa", "sher", "tong", "niu", "nin", "jei",
    "rou", "guai", "da", "zhuo", "yer", "deng", "mo", "ciu", "shai", "sho", "rin", "zao", "mui",
    "ou", "cui", "cun", "suang", "gie", "cuan", "zhuan", "chan", "zhing", "dai", "ni", "wer",
    "wong", "rong", "nang", "qang", "wai", "na", "ye", "ten", "zv", "pen", "zuang", "fer", "no",
    "ria", "zhao", "xa", "geng", "pei", "buo", "ci", "jie", "siu", "bing", "cao", "cuo", "fe",
    "yang", "zo", "muo", "chai", "yo", "chi", "tuan", "wiu", "song", "dia", "nuan", "huo", "bun",
    "bong", "rv", "en", "fei", "bo", "ja", "shia", "mai", "gai", "ki", "fa", "gao", "men", "xie",
    "zhve", "jun", "qo", "xuang", "fou", "shiu", "kv", "lin", "ru", "rve", "qen", "cho", "yie",
    "ging", "xou", "ruang", "jai", "pai", "zou", "zhua", "pi", "pua", "jao", "hong", "wang", "shu",
    "qei", "fuang", "shin", "jer", "kuan", "jen", "ha", "mie", "chia", "shuan", "zhan", "xui",
    "ziu", "diu", "hun", "chang", "kai", "chua", "yue", "yi", "mei", "xun", "ban", "fui", "zhun",
    "yuan", "mong", "rua", "piu", "ler", "er", "zhiu", "jan", "zhuang", "di", "dua", "duang",
    "chui", "miu", "tuo", "qve", "e", "quai", "nao", "gi", "ning", "ang", "xei", "ding", "xen",
    "king", "cher", "hen", "reng", "yei", "muai", "zve", "cei", "xai", "chao", "ying", "yao",
    "chei", "qong", "jing", "wo", "ruai", "wv", "nu", "chuan", "xin", "per", "chuo", "jve", "rai",
    "chen", "du", "seng", "rie", "shuai", "mve", "ai", "heng", "juai", "qin", "sen", "sha", "rer",
    "pui", "zhv", "dang", "zi", "puo", "can", "ga", "wu", "wua", "lan", "zhia", "tia", "pong",
    "cing", "min", "zhei", "yu", "nei", "sei", "shv", "zher", "zui", "huang", "shao", "dve",
    "zhui", "bin", "tei", "zin", "hing", "shuang", "nou", "zei", "wuo", "gua", "kuo", "hei", "sve",
    "cong", "ma", "zong", "chie", "nia", "die", "hv", "zhi", "nai", "xv", "zai", "chuai", "mou",
    "xi", "rui", "zheng", "yui", "mer", "she", "qou", "dun", "zhu", "zang", "zuo", "hi", "giu",
    "yua", "ge", "shing", "ku", "cin", "hou", "sang", "le", "xiu", "li", "sou", "gve", "qan",
    "luai", "tu", "jui", "shong", "zan", "zhou", "ring", "shve", "teng", "kuai", "jo", "zeng",
    "nuang", "zu", "fen", "wuai", "tve", "neng", "zhin", "sie", "qui", "jv", "hie", "re", "muan",
    "fiu", "wun", "hao", "fuan", "gun", "gv", "wie", "shou", "zho", "ching", "fia", "juang", "riu",
    "wan", "wi", "suo", "sun", "ri", "xer", "jou", "shang", "peng", "qi", "lao", "wuan", "de",
    "pang", "pia", "lei", "ro", "din", "chu", "kao", "pve", "chin", "ei", "tv", "xuai", "ming",
    "gin", "xuo", "quo", "bu", "nen", "kua", "puan", "suai", "qai", "tie", "quang", "nong", "ju",
    "qua", "go", "rao", "do", "cheng", "ke", "fang", "sai", "ner", "to", "tao", "tin", "xuan",
    "cang", "shua", "han", "we", "ng", "wuang", "yun", "chiu", "te", "dui", "fua", "shuo", "xe",
    "fao", "ao", "chuang", "ben", "cou", "chou", "fai", "fuai", "you", "xeng", "cai", "qia", "lo",
    "run", "zha", "yiu", "mua", "sv", "lve", "qe", "la", "zie", "pou", "mang", "wing", "tuang",
    "zhuai", "bou", "fong", "hng", "hui", "zhang", "yeng", "tou", "jin", "pe", "nui", "tang", "ka",
    "buang", "ze", "ker", "sin", "kve", "fan", "kou", "jeng", "qiu", "gan", "qing", "shi", "luan",
    "wve", "shun", "shie", "xve", "ho", "rei", "bia", "gang", "pao", "ran", "liu", "quan", "qer",
    "sao", "san", "len", "juan", "ceng", "sing", "shen", "me", "yuang", "gei", "lie", "rang",
    "ling", "tiu", "xang", "be", "zing", "qu", "kia", "cu", "xo", "xong", "qie", "bie", "huan",
    "hia", "xu", "pu", "ting", "gui", "tuai", "fin", "guang", "qun", "ko", "mao", "mia", "huai",
    "ping", "bve", "fu", "ra", "zhie", "bei", "jia", "ken", "zua", "lun", "bui", "cua", "zun",
    "ter", "nie", "yin", "den", "luo", "nua", "long", "yong", "kei",
];

pub fn py_split(word: &str, word_list: &[&str]) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    dp(&mut res, word, word_list, "".to_string());
    res.sort_by_key(|x| x.len());
    res
}

fn dp(res: &mut Vec<Vec<String>>, word: &str, word_list: &[&str], pinyin_list_str: String) {
    let len = word.len();
    for i in 0..=len {
        let mut p_list: Vec<String> = pinyin_list_str.split(',').map(|x| x.to_string()).collect();
        let sub_word = word[0..i].to_string();
        if word_list.contains(&sub_word.as_str()) {
            if i == len {
                p_list.push(sub_word);
                res.push(p_list[1..].iter().map(|x| x.to_string()).collect());
            } else {
                p_list.push(sub_word);
                dp(res, &word[i..], word_list, p_list.join(","));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinyin_split() {
        let result = py_split("nihaoa", &POSIBLE_PINYINS);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec!["ni".to_string(), "hao".to_string(), "a".to_string()]));
        assert!(result.contains(&vec![
            "ni".to_string(),
            "ha".to_string(),
            "o".to_string(),
            "a".to_string()
        ]));
    }
}
