use roxmltree::Node;

pub(crate) fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("name cannot be empty".to_string());
    }
    if name.chars().nth(0).unwrap().is_numeric() {
        return Err("name cannot start with a number".to_string());
    }
    if name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        return Err("name can only have alphanumeric characters and underscores and have no spaces".to_string());
    }

    return Ok(());
}

pub(crate) fn node_pos(node: &Node) -> String {
    let pos = node.document().text_pos_at(node.range().start);
    format!("line {}, column {}", pos.row, pos.col)
}

pub(crate) fn strip_prefixes(id: &str, prefixes: &Vec<String>) -> String {
    let mut name = id;

    for prefix in prefixes {
        if let Some(stripped) = id.strip_prefix(prefix) {
            name = stripped;
            break;
        }
    }

    name = name.trim_matches('_');

    name.into()
}

pub(crate) fn is_integral_range(min: i32, max: i32, div: i32) -> bool {
    (max - min) % div == 0
} 

pub(crate) fn common_str_prefix(v: &Vec<String>) -> &str {
    let mut common_len = usize::MAX; 
    for i in 0..v.len() - 1 {
        let s1 = &v[i];
        let s2 = &v[i + 1];
        common_len = std::cmp::min(common_len, common_str_prefix_len(s1, s2));
    }

    if common_len > 0 {
        &v[0][0..common_len]
    } else {
        ""
    }
}

fn common_str_prefix_len(s1: &str, s2: &str) -> usize {
    let min_len = std::cmp::min(s1.len(), s2.len());

    for i in 0..min_len {
        if s1.chars().nth(i) != s2.chars().nth(i) {
            return i;
        }
    }

    min_len
}