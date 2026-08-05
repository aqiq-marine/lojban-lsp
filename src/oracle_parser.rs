#[derive(Debug, PartialEq, Eq)]
pub enum JSOutputNode {
    Node(String, Vec<JSOutputNode>),
    Leaf(String),
}

pub fn parse_js_output(input: &str) -> JSOutputNode {
    let tokens = tokenize(input);
    let mut pos = 0;
    parse_node(&tokens, &mut pos)
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for c in input.chars() {
        if c.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current.split_off(0));
            }
        } else if c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}' || c == '<' || c == '>' {
            if !current.is_empty() {
                tokens.push(current.split_off(0));
            }
            tokens.push(c.to_string());
        } else {
            current.push(c);
        }
    }
    tokens
}

fn parse_node(tokens: &[String], pos: &mut usize) -> JSOutputNode {
    let mut children = Vec::new();
    let mut name = String::new();

    while *pos < tokens.len() {
        let t = &tokens[*pos];
        match t.as_str() {
            "(" | "[" | "{" | "<" => {
                *pos += 1;
                children.push(parse_node(tokens, pos));
            }
            ")" | "]" | "}" | ">" => {
                *pos += 1;
                return JSOutputNode::Node(name, children);
            }
            _ => {
                if name.is_empty() {
                    name = t.clone();
                } else {
                    children.push(JSOutputNode::Leaf(t.clone()));
                }
                *pos += 1;
            }
        }
    }
    JSOutputNode::Node(name, children)
}
