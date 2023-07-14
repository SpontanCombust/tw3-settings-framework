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

pub(crate) fn id_to_script_name(id: &str, omit_prefix: &Option<String>) -> String {
    if let Some(prefix) = omit_prefix {
        if id.starts_with(prefix) {
            return id[prefix.len()..].to_string();
        }
    } 

    return id.to_string();
}