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

pub(crate) fn id_to_script_name(id: &str, omit_prefixes: &Vec<String>) -> String {
    let mut name = id;

    for prefix in omit_prefixes {
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