//! c4-diagrams

use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fmt::{Display, Write},
};

use crate::args::OutputFormat;

pub type Id = String;

pub type Text = String;

pub enum ElementKind {
    Container,
    ContainerDatabase,
    ExternalContainer,
}

impl Display for ElementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ElementKind::*;
        let str = match self {
            Container => "Container",
            ContainerDatabase => "ContainerDb",
            ExternalContainer => "ExternalContainer",
        };

        write!(f, "{str}")
    }
}

pub struct Element {
    pub kind: ElementKind,
    pub id: Id,
    pub title: Text,
    pub description: Text,
}

#[derive(Clone)]
pub enum RelationshipKind {
    BiRel,
    Rel,
}

impl Display for RelationshipKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RelationshipKind::*;
        let s = match self {
            Rel => "Rel",
            BiRel => "BiRel",
        };

        write!(f, "{s}")
    }
}

pub struct Diagram {
    title: String,
    elements: BTreeMap<Id, Element>,
    relationships: HashMap<Id, Vec<Id>>,
    filter: HashSet<String>,
}

impl Diagram {
    pub fn new(title: String, filter_str: String) -> Self {
        Self {
            title,
            elements: BTreeMap::new(),
            relationships: HashMap::new(),
            filter: filter_str.split(',').map(|s| s.to_string()).collect(),
        }
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.insert(element.id.clone(), element);
    }

    pub fn has_element(&self, id: &Id) -> bool {
        self.elements.contains_key(id)
    }

    pub fn add_relationship(&mut self, from: Id, to: Id) {
        self.relationships
            .entry(from)
            .or_insert(Vec::new())
            .push(to);
    }

    pub fn render(&mut self, output_format: OutputFormat) -> Result<String, std::fmt::Error> {
        match output_format {
            OutputFormat::C4 => self.render_c4(),
            OutputFormat::Graphviz => self.render_graphviz(),
        }
    }

    pub fn render_c4(&self) -> Result<String, std::fmt::Error> {
        //! Renders a diagram in the MermaidJS C4 Syntax. \
        //! See <https://mermaid.js.org/syntax/c4.html> for more info.

        let mut output = String::new();

        writeln!(output, "C4Context")?;
        writeln!(output, "\ttitle {}", self.title)?;

        for element in self.elements.values() {
            if self.filter.contains(&element.id) {
                continue;
            }
            writeln!(
                output,
                "\t{kind}({id}, \"{title}\", \"{description}\")",
                kind = element.kind,
                id = element.id,
                title = element.title,
                description = element.description,
            )?;
        }

        for (from, tos) in self.relationships.iter() {
            if self.filter.contains(from) {
                continue;
            }
            for to in tos {
                if !self.filter.contains(to) {
                    writeln!(
                        output,
                        "\t{kind}({from}, {to}, \"depends\")",
                        kind = RelationshipKind::Rel,
                    )?;
                }
            }
        }

        Ok(output)
    }

    pub fn render_graphviz(&mut self) -> Result<String, std::fmt::Error> {
        //! Renders a diagram in the Graphviz DOT format. \
        //! See <https://graphviz.org/doc/info/lang.html> for more info.
        //!
        //! Viewable on <https://dreampuf.github.io/GraphvizOnline/>

        let mut output = String::new();

        writeln!(output, "digraph {title} {{", title = self.title)?;

        for (from, tos) in self.relationships.iter() {
            if self.filter.contains(from) {
                continue;
            }
            for to in tos {
                if !self.filter.contains(to) {
                    writeln!(output, "  \"{from}\" -> \"{to}\"")?;
                }
            }
        }

        writeln!(output, "}}")?;

        Ok(output)
    }
}
