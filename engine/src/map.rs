//! A story's scene graph as a diagram: Mermaid (shows up rendered on GitHub
//! and in many editors) or Graphviz DOT. Endings are rounded, the start is
//! marked, choices with a condition are dashed, unreachable scenes are dimmed.

use crate::pack::StoryPack;

/// Diagram formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Mermaid,
    Dot,
}

/// Choice labels are cut to this many characters.
const LABEL: usize = 40;

fn short(text: &str) -> String {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.chars().count() <= LABEL {
        text
    } else {
        let cut: String = text.chars().take(LABEL - 1).collect();
        format!("{}…", cut.trim_end())
    }
}

/// `the_office` → `The Office`.
fn title(id: &str) -> String {
    id.split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn render(pack: &StoryPack, format: Format) -> String {
    match format {
        Format::Mermaid => mermaid(pack),
        Format::Dot => dot(pack),
    }
}

fn gated(c: &crate::scene::Choice) -> bool {
    c.cond.is_some() || !c.any_flags.is_empty() || !c.any_items.is_empty()
}

fn mermaid(pack: &StoryPack) -> String {
    // node ids get a prefix: Mermaid has reserved words such as `end`
    let node = |id: &str| format!("s_{id}");
    let esc = |s: &str| s.replace('"', "#quot;");
    let unreachable = pack.unreachable();
    let mut out = format!("---\ntitle: {}\n---\nflowchart TD\n", esc(&pack.meta.title));
    for (id, scene) in &pack.scenes {
        let label = esc(&title(id));
        let shape = if scene.ending || scene.choices.is_empty() {
            format!("([\"{label}\"])")
        } else {
            format!("[\"{label}\"]")
        };
        let class = if id == &pack.meta.start {
            ":::start"
        } else if unreachable.contains(&id.as_str()) {
            ":::unreachable"
        } else if scene.ending || scene.choices.is_empty() {
            ":::ending"
        } else {
            ""
        };
        out += &format!("    {}{shape}{class}\n", node(id));
    }
    for (id, scene) in &pack.scenes {
        for c in &scene.choices {
            let arrow = if gated(c) { "-.->" } else { "-->" };
            out += &format!(
                "    {} {arrow}|\"{}\"| {}\n",
                node(id),
                esc(&short(&c.text)),
                node(&c.goto)
            );
        }
    }
    out += "    classDef start fill:#123d2a,stroke:#3ddc84,color:#fff\n";
    out += "    classDef ending fill:#3d1224,stroke:#ff2a6d,color:#fff\n";
    out += "    classDef unreachable fill:#222,stroke:#666,color:#888,stroke-dasharray:4\n";
    out
}

fn dot(pack: &StoryPack) -> String {
    let esc = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");
    let unreachable = pack.unreachable();
    let mut out = format!(
        "digraph \"{}\" {{\n    rankdir=TB;\n    node [shape=box, style=\"rounded,filled\", fillcolor=\"#1d1d28\", fontcolor=white, color=\"#555\"];\n    edge [color=\"#888\", fontsize=10];\n",
        esc(&pack.meta.title)
    );
    for (id, scene) in &pack.scenes {
        let mut attrs = vec![format!("label=\"{}\"", esc(&title(id)))];
        if id == &pack.meta.start {
            attrs.push("fillcolor=\"#123d2a\", color=\"#3ddc84\"".into());
        } else if unreachable.contains(&id.as_str()) {
            attrs.push("style=\"rounded,dashed\", fontcolor=\"#888\"".into());
        } else if scene.ending || scene.choices.is_empty() {
            attrs.push("shape=ellipse, fillcolor=\"#3d1224\", color=\"#ff2a6d\"".into());
        }
        out += &format!("    \"{}\" [{}];\n", esc(id), attrs.join(", "));
    }
    for (id, scene) in &pack.scenes {
        for c in &scene.choices {
            let style = if gated(c) { ", style=dashed" } else { "" };
            out += &format!(
                "    \"{}\" -> \"{}\" [label=\"{}\"{style}];\n",
                esc(id),
                esc(&c.goto),
                esc(&short(&c.text))
            );
        }
    }
    out += "}\n";
    out
}
