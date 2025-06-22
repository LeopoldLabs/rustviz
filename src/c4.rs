//! c4-diagrams

use std::{collections::BTreeMap, fmt::Display};

pub type Id = rustdoc_types::Id;

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

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{kind}({id}, \"{title}\", \"{description}\")",
            kind = self.kind,
            id = self.id.0,
            title = self.title,
            description = self.description,
        )
    }
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

#[derive(Clone)]
pub struct Relationship {
    pub kind: RelationshipKind,
    pub from: Id,
    pub to: Id,
    pub description: Text,
}

impl Display for Relationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{kind}({from}, {to}, \"{description}\")",
            kind = self.kind,
            from = self.from.0,
            to = self.to.0,
            description = self.description,
        )
    }
}

pub struct Diagram {
    title: String,
    elements: BTreeMap<Id, Element>,
    relationships: Vec<Relationship>,
}

impl Diagram {
    pub fn new(title: String) -> Self {
        Self {
            title,
            elements: BTreeMap::new(),
            relationships: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: Element) {
        let returning = self.elements.insert(element.id, element);
        assert!(returning.is_none());
    }

    pub fn add_relationship(&mut self, relation: Relationship) {
        self.relationships.push(relation);
    }

    pub fn clean(&mut self) {
        self.relationships = self
            .relationships
            .iter()
            .cloned()
            .filter(|r| self.elements.contains_key(&r.from) && self.elements.contains_key(&r.to))
            .collect()
    }
}

impl Display for Diagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //! Renders a diagram in the MermaidJS C4 Syntax. \
        //! See <https://mermaid.js.org/syntax/c4.html> for more info.

        writeln!(f, "C4Context")?;
        writeln!(f, "\ttitle {}", self.title)?;

        for element in self.elements.values() {
            writeln!(f, "\t{element}")?;
        }

        for relationship in self.relationships.iter() {
            writeln!(f, "\t{relationship}")?;
        }

        Ok(())
    }
}
