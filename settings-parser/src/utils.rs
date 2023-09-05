use std::str::FromStr;

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

pub(crate) fn strip_prefixes<'a>(s: &'a str, prefixes: &'a [String]) -> &'a str {
    let mut stripped_s = s;

    for prefix in prefixes {
        if let Some(stripped) = stripped_s.strip_prefix(prefix) {
            stripped_s = stripped;
            break;
        }
    }

    stripped_s
}

pub(crate) fn is_integral_range(min: i32, max: i32, div: i32) -> bool {
    (max - min) % div == 0
} 

pub(crate) fn common_str_prefix(v: &[String]) -> &str {
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


pub fn parse_attribute_string(node: &Node, attr: &'static str, validate: bool) -> Result<Option<String>, String> {
    let v = node.attribute(attr);

    if let Some(v) = v {
        if validate {
            validate_name(v)?;
        }

        Ok(Some(v.to_string()))
    } else {
        Ok(None)
    }
}

pub fn parse_attribute_number<N: FromStr>(node: &Node, attr: &'static str) -> Result<Option<N>, String> {
    let v = node.attribute(attr);

    if let Some(v) = v {
        match str::parse::<N>(v) {
            Ok(vn) => Ok(Some(vn)),
            Err(_) => Err(format!("Invalid {} attribute at {}: expected a number", attr, node_pos(node)))
        }
    } else {
        Ok(None)
    }
}

pub fn parse_attribute_bool(node: &Node, attr: &'static str) -> Result<Option<bool>, String> {
    let v = node.attribute(attr);

    if let Some(v) = v {
        match v {
            "true" => Ok(Some(true)),
            "false" => Ok(Some(false)),
            _ => Err(format!("Invalid value for attribute {} at {}", attr, node_pos(node)))
        }
    } else {
        Ok(None)
    }
}


pub fn parse_attribute_string_required(node: &Node, attr: &'static str, validate: bool) -> Result<String, String> {
    let val = parse_attribute_string(node, attr, validate)?;
    if let Some(val) = val {
        Ok(val)
    } else {
        Err(format!("{} node at {} is missing {} attribute", node.tag_name().name(), node_pos(node), attr))
    }
}

pub fn parse_attribute_number_required<N: FromStr>(node: &Node, attr: &'static str) -> Result<N, String> {
    let val = parse_attribute_number(node, attr)?;
    if let Some(val) = val {
        Ok(val)
    } else {
        Err(format!("{} node at {} is missing {} attribute", node.tag_name().name(), node_pos(node), attr))
    }
}

// pub fn parse_attribute_bool_required(node: &Node, attr: &'static str) -> Result<bool, String> {
//     let val = parse_attribute_bool(node, attr)?;
//     if let Some(val) = val {
//         Ok(val)
//     } else {
//         Err(format!("{} node at {} is missing {} attribute", node.tag_name().name(), node_pos(node), attr))
//     }
// }