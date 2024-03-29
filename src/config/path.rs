use crate::config::error::*;
use crate::config::value::{Value, ValueKind};

use std::collections::HashMap;
use std::str::FromStr;

mod parser;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Expression {
    Identifier(String),
    Child(Box<Expression>, String),
    Subscript(Box<Expression>, isize),
}

impl FromStr for Expression {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Expression> {
        parser::from_str(s).map_err(ConfigError::PathParse)
    }
}

fn sindex_to_uindex(index: isize, len: usize) -> usize {
    if index >= 0 {
        index as usize
    } else {
        len - (index.unsigned_abs())
    }
}

impl Expression {
    pub fn get_mut_forcibly<'a>(&self, root: &'a mut Value) -> Option<&'a mut Value> {
        match *self {
            Expression::Identifier(ref id) => match root.kind {
                ValueKind::Table(ref mut map) => Some(
                    map.entry(id.clone())
                        .or_insert_with(|| Value::new(None, ValueKind::Nil)),
                ),

                _ => None,
            },

            Expression::Child(ref expr, ref key) => match expr.get_mut_forcibly(root) {
                Some(value) => match value.kind {
                    ValueKind::Table(ref mut map) => Some(
                        map.entry(key.clone())
                            .or_insert_with(|| Value::new(None, ValueKind::Nil)),
                    ),

                    _ => {
                        *value = HashMap::<String, Value>::new().into();

                        if let ValueKind::Table(ref mut map) = value.kind {
                            Some(
                                map.entry(key.clone())
                                    .or_insert_with(|| Value::new(None, ValueKind::Nil)),
                            )
                        } else {
                            unreachable!();
                        }
                    }
                },

                _ => None,
            },

            Expression::Subscript(ref expr, index) => match expr.get_mut_forcibly(root) {
                Some(value) => {
                    match value.kind {
                        ValueKind::Array(_) => (),
                        _ => *value = Vec::<Value>::new().into(),
                    }

                    match value.kind {
                        ValueKind::Array(ref mut array) => {
                            let index = sindex_to_uindex(index, array.len());

                            if index >= array.len() {
                                array.resize(index + 1, Value::new(None, ValueKind::Nil));
                            }

                            Some(&mut array[index])
                        }

                        _ => None,
                    }
                }
                _ => None,
            },
        }
    }

    pub fn set(&self, root: &mut Value, value: Value) {
        match *self {
            Expression::Identifier(ref id) => {
                // Ensure that root is a table.
                match root.kind {
                    ValueKind::Table(_) => {}

                    _ => {
                        *root = HashMap::<String, Value>::new().into();
                    }
                }

                match value.kind {
                    ValueKind::Table(ref incoming_map) => {
                        // Pull out another table.
                        let target = if let ValueKind::Table(ref mut map) = root.kind {
                            map.entry(id.clone())
                                .or_insert_with(|| HashMap::<String, Value>::new().into())
                        } else {
                            unreachable!();
                        };

                        // Continue the deep merge.
                        for (key, val) in incoming_map {
                            Expression::Identifier(key.clone()).set(target, val.clone());
                        }
                    }

                    _ => {
                        if let ValueKind::Table(ref mut map) = root.kind {
                            // Just do a simple set.
                            map.insert(id.clone(), value);
                        }
                    }
                }
            }

            Expression::Child(ref expr, ref key) => {
                if let Some(parent) = expr.get_mut_forcibly(root) {
                    match parent.kind {
                        ValueKind::Table(_) => {
                            Expression::Identifier(key.clone()).set(parent, value);
                        }

                        _ => {
                            // Didn't find a table. Oh well. Make a table and do this anyway.
                            *parent = HashMap::<String, Value>::new().into();

                            Expression::Identifier(key.clone()).set(parent, value);
                        }
                    }
                }
            }

            Expression::Subscript(ref expr, index) => {
                if let Some(parent) = expr.get_mut_forcibly(root) {
                    match parent.kind {
                        ValueKind::Array(_) => (),
                        _ => *parent = Vec::<Value>::new().into(),
                    }

                    if let ValueKind::Array(ref mut array) = parent.kind {
                        let uindex = sindex_to_uindex(index, array.len());
                        if uindex >= array.len() {
                            array.resize(uindex + 1, Value::new(None, ValueKind::Nil));
                        }

                        array[uindex] = value;
                    }
                }
            }
        }
    }
}
