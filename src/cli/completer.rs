use reedline::{Completer, Span, Suggestion};
use std::sync::{Arc, Mutex};

pub struct CommandCompleter {
    pub names: Arc<Mutex<Vec<String>>>,
}

impl Completer for CommandCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let names = self.names.lock().unwrap();

        match parts.as_slice() {
            [] => all_commands(pos),
            [cmd] if !line.ends_with(' ') => filter_commands(cmd, pos),
            ["show", _] | ["initiative", _] if line.ends_with(' ') => {
                filter_names(&names, "", line, pos)
            }
            ["show", prefix] => filter_names(&names, prefix, line, pos),
            ["initiative", prefix] => filter_names(&names, prefix, line, pos),
            ["load", prefix] => complete_paths(prefix, line, pos),
            _ => vec![],
        }
    }
}

fn all_commands(pos: usize) -> Vec<Suggestion> {
    filter_commands("", pos)
}

fn filter_commands(prefix: &str, pos: usize) -> Vec<Suggestion> {
    let commands = &[
        "load",
        "list",
        "show",
        "initiative",
        "order",
        "next",
        "quit",
        "exit",
    ];
    commands
        .iter()
        .filter(|cmd| cmd.starts_with(prefix))
        .map(|cmd| Suggestion {
            value: cmd.to_string(),
            description: None,
            extra: None,
            span: Span::new(0, pos),
            append_whitespace: true,
            display_override: None,
            style: None,
            match_indices: None,
        })
        .collect()
}

fn filter_names(names: &[String], prefix: &str, line: &str, pos: usize) -> Vec<Suggestion> {
    let start = line.len() - prefix.len();
    names
        .iter()
        .filter(|n| n.to_lowercase().starts_with(&prefix.to_lowercase()))
        .map(|n| Suggestion {
            value: n.clone(),
            description: None,
            extra: None,
            span: Span::new(start, pos),
            append_whitespace: true,
            display_override: None,
            style: None,
            match_indices: None,
        })
        .collect()
}

fn complete_paths(prefix: &str, line: &str, pos: usize) -> Vec<Suggestion> {
    let (dir, file_prefix) = match prefix.rfind('/') {
        Some(i) => (&prefix[..=i], &prefix[i + 1..]),
        None => (".", prefix),
    };

    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };

    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(file_prefix))
                .unwrap_or(false)
        })
        .map(|p| {
            let start = line.len() - prefix.len();
            Suggestion {
                value: p.to_string_lossy().to_string(),
                description: None,
                extra: None,
                span: Span::new(start, pos),
                append_whitespace: false,
                display_override: None,
                style: None,
                match_indices: None,
            }
        })
        .collect()
}
