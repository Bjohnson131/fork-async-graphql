
//! This is @generated code, do not edit by hand.
//! See `graphql.pest` and `tests/codegen.rs`.
#![allow(unused_attributes)]
use super::GraphQLParser;

#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    r#WHITESPACE,
    r#COMMENT,
    r#line_terminator,
    r#executable_document,
    r#executable_definition,
    r#operation_definition,
    r#named_operation_definition,
    r#variable_definitions,
    r#variable_definition,
    r#selection_set,
    r#selection,
    r#field,
    r#alias,
    r#fragment_spread,
    r#inline_fragment,
    r#fragment_definition,
    r#type_condition,
    r#service_document,
    r#type_system_definition,
    r#schema_definition,
    r#operation_type_definition,
    r#type_definition,
    r#scalar_type,
    r#object_type,
    r#implements_interfaces,
    r#interface_type,
    r#fields_definition,
    r#field_definition,
    r#union_type,
    r#union_member_types,
    r#enum_type,
    r#enum_values,
    r#enum_value_definition,
    r#input_object_type,
    r#input_fields_definition,
    r#extend,
    r#directive_definition,
    r#directive_locations,
    r#directive_location,
    r#arguments_definition,
    r#input_value_definition,
    r#operation_type,
    r#default_value,
    r#type_,
    r#const_value,
    r#value,
    r#variable,
    r#number,
    r#float,
    r#fractional,
    r#exponent,
    r#int,
    r#string,
    r#block_string_content,
    r#block_string_character,
    r#string_content,
    r#string_character,
    r#unicode_scalar_value_hex,
    r#boolean,
    r#null,
    r#enum_value,
    r#const_list,
    r#list,
    r#const_object,
    r#object,
    r#const_object_field,
    r#object_field,
    r#const_directives,
    r#directives,
    r#const_directive,
    r#directive,
    r#const_arguments,
    r#arguments,
    r#const_argument,
    r#argument,
    r#name_start,
    r#name,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for GraphQLParser {
    fn parse<'i>(
        rule: Rule,
        input: &'i str,
    ) -> ::std::result::Result<
        ::pest::iterators::Pairs<'i, Rule>,
        ::pest::error::Error<Rule>,
    > {
        mod rules {
            #![allow(clippy::upper_case_acronyms)]
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state
                            .sequence(|state| {
                                state
                                    .repeat(|state| super::visible::WHITESPACE(state))
                                    .and_then(|state| {
                                        state
                                            .repeat(|state| {
                                                state
                                                    .sequence(|state| {
                                                        super::visible::COMMENT(state)
                                                            .and_then(|state| {
                                                                state.repeat(|state| super::visible::WHITESPACE(state))
                                                            })
                                                    })
                                            })
                                    })
                            })
                    } else {
                        Ok(state)
                    }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#WHITESPACE(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::Atomic,
                            |state| {
                                state
                                    .match_string(" ")
                                    .or_else(|state| { state.match_string(",") })
                                    .or_else(|state| { state.match_string("\t") })
                                    .or_else(|state| { state.match_string("\u{feff}") })
                                    .or_else(|state| { self::r#line_terminator(state) })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#COMMENT(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::Atomic,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("#")
                                            .and_then(|state| {
                                                state
                                                    .repeat(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .lookahead(
                                                                        false,
                                                                        |state| { self::r#line_terminator(state) },
                                                                    )
                                                                    .and_then(|state| { self::r#ANY(state) })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#line_terminator(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#line_terminator,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            state
                                                .match_string("\r\n")
                                                .or_else(|state| { state.match_string("\r") })
                                                .or_else(|state| { state.match_string("\n") })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#executable_document(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#executable_document,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#SOI(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#executable_definition(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#executable_definition(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#executable_definition(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#EOI(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#executable_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#executable_definition,
                            |state| {
                                self::r#operation_definition(state)
                                    .or_else(|state| { self::r#fragment_definition(state) })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#operation_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#operation_definition,
                            |state| {
                                self::r#named_operation_definition(state)
                                    .or_else(|state| { self::r#selection_set(state) })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#named_operation_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#named_operation_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#operation_type(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#name(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .optional(|state| { self::r#variable_definitions(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#selection_set(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#variable_definitions(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#variable_definitions,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("(")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#variable_definition(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#variable_definition(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(")") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#variable_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#variable_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#variable(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#type_(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#default_value(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#selection_set(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#selection_set,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("{")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#selection(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#selection(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#selection(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("}") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#selection(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#selection,
                            |state| {
                                self::r#field(state)
                                    .or_else(|state| { self::r#inline_fragment(state) })
                                    .or_else(|state| { self::r#fragment_spread(state) })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#field(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#field,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#alias(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#arguments(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#selection_set(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#alias(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#alias,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#name(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#fragment_spread(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#fragment_spread,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("...")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .lookahead(false, |state| { self::r#type_condition(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#directives(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#inline_fragment(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#inline_fragment,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("...")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#type_condition(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#selection_set(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#fragment_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#fragment_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("fragment")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#type_condition(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#selection_set(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#type_condition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::CompoundAtomic,
                            |state| {
                                state
                                    .rule(
                                        Rule::r#type_condition,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    state
                                                        .match_string("on")
                                                        .and_then(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    self::r#WHITESPACE(state)
                                                                        .and_then(|state| {
                                                                            state.repeat(|state| { self::r#WHITESPACE(state) })
                                                                        })
                                                                })
                                                        })
                                                        .and_then(|state| { self::r#name(state) })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#service_document(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#service_document,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#SOI(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#type_system_definition(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#type_system_definition(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#type_system_definition(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#EOI(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#type_system_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#type_system_definition,
                            |state| {
                                self::r#schema_definition(state)
                                    .or_else(|state| { self::r#type_definition(state) })
                                    .or_else(|state| { self::r#directive_definition(state) })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#schema_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#schema_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("schema")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("{") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#operation_type_definition(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#operation_type_definition(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| {
                                                                                                                self::r#operation_type_definition(state)
                                                                                                            })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("}") })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                self::r#extend(state)
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { state.match_string("schema") })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .optional(|state| { self::r#const_directives(state) })
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| { state.match_string("{") })
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| {
                                                                        state
                                                                            .sequence(|state| {
                                                                                self::r#operation_type_definition(state)
                                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .sequence(|state| {
                                                                                                state
                                                                                                    .optional(|state| {
                                                                                                        self::r#operation_type_definition(state)
                                                                                                            .and_then(|state| {
                                                                                                                state
                                                                                                                    .repeat(|state| {
                                                                                                                        state
                                                                                                                            .sequence(|state| {
                                                                                                                                super::hidden::skip(state)
                                                                                                                                    .and_then(|state| {
                                                                                                                                        self::r#operation_type_definition(state)
                                                                                                                                    })
                                                                                                                            })
                                                                                                                    })
                                                                                                            })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| { state.match_string("}") })
                                                            })
                                                            .or_else(|state| { self::r#const_directives(state) })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#operation_type_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#operation_type_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#operation_type(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#type_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#type_definition,
                            |state| {
                                self::r#scalar_type(state)
                                    .or_else(|state| { self::r#object_type(state) })
                                    .or_else(|state| { self::r#interface_type(state) })
                                    .or_else(|state| { self::r#union_type(state) })
                                    .or_else(|state| { self::r#enum_type(state) })
                                    .or_else(|state| { self::r#input_object_type(state) })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#scalar_type(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#scalar_type,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("scalar") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                self::r#extend(state)
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { state.match_string("scalar") })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { self::r#name(state) })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { self::r#const_directives(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#object_type(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#object_type,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("type") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .optional(|state| { self::r#implements_interfaces(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#fields_definition(state) })
                                            })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                self::r#extend(state)
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { state.match_string("type") })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { self::r#name(state) })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .optional(|state| { self::r#implements_interfaces(state) })
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| {
                                                                        state
                                                                            .sequence(|state| {
                                                                                state
                                                                                    .optional(|state| { self::r#const_directives(state) })
                                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                                    .and_then(|state| { self::r#fields_definition(state) })
                                                                            })
                                                                            .or_else(|state| { self::r#const_directives(state) })
                                                                    })
                                                            })
                                                            .or_else(|state| { self::r#implements_interfaces(state) })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#implements_interfaces(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#implements_interfaces,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("implements")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { state.match_string("&") })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .match_string("&")
                                                                            .and_then(|state| { super::hidden::skip(state) })
                                                                            .and_then(|state| { self::r#name(state) })
                                                                    })
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        state
                                                                                                            .match_string("&")
                                                                                                            .and_then(|state| { super::hidden::skip(state) })
                                                                                                            .and_then(|state| { self::r#name(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#interface_type(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#interface_type,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("interface") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .optional(|state| { self::r#implements_interfaces(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#fields_definition(state) })
                                            })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                self::r#extend(state)
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { state.match_string("interface") })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { self::r#name(state) })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .optional(|state| { self::r#implements_interfaces(state) })
                                                    })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .optional(|state| { self::r#const_directives(state) })
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| { self::r#fields_definition(state) })
                                                            })
                                                            .or_else(|state| { self::r#const_directives(state) })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#fields_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#fields_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("{")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#field_definition(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#field_definition(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#field_definition(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("}") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#field_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#field_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .optional(|state| { self::r#arguments_definition(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#type_(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#union_type(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#union_type,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("union") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .optional(|state| { self::r#union_member_types(state) })
                                            })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                self::r#extend(state)
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { state.match_string("union") })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { self::r#name(state) })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .optional(|state| { self::r#const_directives(state) })
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| { self::r#union_member_types(state) })
                                                            })
                                                            .or_else(|state| { self::r#const_directives(state) })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#union_member_types(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#union_member_types,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("=")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { state.match_string("|") })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .match_string("|")
                                                                            .and_then(|state| { super::hidden::skip(state) })
                                                                            .and_then(|state| { self::r#name(state) })
                                                                    })
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        state
                                                                                                            .match_string("|")
                                                                                                            .and_then(|state| { super::hidden::skip(state) })
                                                                                                            .and_then(|state| { self::r#name(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#enum_type(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#enum_type,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("enum") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#enum_values(state) })
                                            })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                self::r#extend(state)
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { state.match_string("enum") })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { self::r#name(state) })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .optional(|state| { self::r#const_directives(state) })
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| { self::r#enum_values(state) })
                                                            })
                                                            .or_else(|state| { self::r#const_directives(state) })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#enum_values(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#enum_values,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("{")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#enum_value_definition(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#enum_value_definition(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#enum_value_definition(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("}") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#enum_value_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#enum_value_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#enum_value(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#input_object_type(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#input_object_type,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("input") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .optional(|state| {
                                                        self::r#input_fields_definition(state)
                                                    })
                                            })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                self::r#extend(state)
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { state.match_string("input") })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| { self::r#name(state) })
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .optional(|state| { self::r#const_directives(state) })
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| {
                                                                        self::r#input_fields_definition(state)
                                                                    })
                                                            })
                                                            .or_else(|state| { self::r#const_directives(state) })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#input_fields_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#input_fields_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("{")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#input_value_definition(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#input_value_definition(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#input_value_definition(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("}") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#extend(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.rule(Rule::r#extend, |state| { state.match_string("extend") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#directive_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#directive_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("directive") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("@") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .optional(|state| { self::r#arguments_definition(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("on") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#directive_locations(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#directive_locations(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#directive_locations,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { state.match_string("|") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#directive_location(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .match_string("|")
                                                                            .and_then(|state| { super::hidden::skip(state) })
                                                                            .and_then(|state| { self::r#directive_location(state) })
                                                                    })
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        state
                                                                                                            .match_string("|")
                                                                                                            .and_then(|state| { super::hidden::skip(state) })
                                                                                                            .and_then(|state| { self::r#directive_location(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#directive_location(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#directive_location,
                            |state| {
                                state
                                    .match_string("QUERY")
                                    .or_else(|state| { state.match_string("MUTATION") })
                                    .or_else(|state| { state.match_string("SUBSCRIPTION") })
                                    .or_else(|state| { state.match_string("FIELD_DEFINITION") })
                                    .or_else(|state| { state.match_string("FIELD") })
                                    .or_else(|state| {
                                        state.match_string("FRAGMENT_DEFINITION")
                                    })
                                    .or_else(|state| { state.match_string("FRAGMENT_SPREAD") })
                                    .or_else(|state| { state.match_string("INLINE_FRAGMENT") })
                                    .or_else(|state| {
                                        state.match_string("VARIABLE_DEFINITION")
                                    })
                                    .or_else(|state| { state.match_string("SCHEMA") })
                                    .or_else(|state| { state.match_string("SCALAR") })
                                    .or_else(|state| { state.match_string("OBJECT") })
                                    .or_else(|state| {
                                        state.match_string("ARGUMENT_DEFINITION")
                                    })
                                    .or_else(|state| { state.match_string("INTERFACE") })
                                    .or_else(|state| { state.match_string("UNION") })
                                    .or_else(|state| { state.match_string("ENUM_VALUE") })
                                    .or_else(|state| { state.match_string("ENUM") })
                                    .or_else(|state| { state.match_string("INPUT_OBJECT") })
                                    .or_else(|state| {
                                        state.match_string("INPUT_FIELD_DEFINITION")
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#arguments_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#arguments_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("(")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#input_value_definition(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#input_value_definition(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#input_value_definition(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(")") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#input_value_definition(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#input_value_definition,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { self::r#string(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#type_(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#default_value(state) })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_directives(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#operation_type(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#operation_type,
                            |state| {
                                state
                                    .match_string("query")
                                    .or_else(|state| { state.match_string("mutation") })
                                    .or_else(|state| { state.match_string("subscription") })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#default_value(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#default_value,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("=")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#const_value(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#type_(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#type_,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    self::r#name(state)
                                                        .or_else(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    state
                                                                        .match_string("[")
                                                                        .and_then(|state| { self::r#type_(state) })
                                                                        .and_then(|state| { state.match_string("]") })
                                                                })
                                                        })
                                                        .and_then(|state| {
                                                            state.optional(|state| { state.match_string("!") })
                                                        })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#const_value(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#const_value,
                            |state| {
                                self::r#number(state)
                                    .or_else(|state| { self::r#string(state) })
                                    .or_else(|state| { self::r#boolean(state) })
                                    .or_else(|state| { self::r#null(state) })
                                    .or_else(|state| { self::r#enum_value(state) })
                                    .or_else(|state| { self::r#const_list(state) })
                                    .or_else(|state| { self::r#const_object(state) })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#value(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#value,
                            |state| {
                                self::r#variable(state)
                                    .or_else(|state| { self::r#number(state) })
                                    .or_else(|state| { self::r#string(state) })
                                    .or_else(|state| { self::r#boolean(state) })
                                    .or_else(|state| { self::r#null(state) })
                                    .or_else(|state| { self::r#enum_value(state) })
                                    .or_else(|state| { self::r#list(state) })
                                    .or_else(|state| { self::r#object(state) })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#variable(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#variable,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("$")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#number(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#number,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    self::r#float(state)
                                                        .or_else(|state| { self::r#int(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .lookahead(false, |state| { self::r#name_start(state) })
                                                        })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#float(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#float,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#int(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#fractional(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| { self::r#exponent(state) })
                                                    })
                                                    .or_else(|state| { self::r#fractional(state) })
                                                    .or_else(|state| { self::r#exponent(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#fractional(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#fractional,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string(".")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ASCII_DIGIT(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#ASCII_DIGIT(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#ASCII_DIGIT(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#exponent(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#exponent,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("E")
                                            .or_else(|state| { state.match_string("e") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .optional(|state| {
                                                        state
                                                            .match_string("+")
                                                            .or_else(|state| { state.match_string("-") })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ASCII_DIGIT(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#ASCII_DIGIT(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#ASCII_DIGIT(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#int(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#int,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .optional(|state| { state.match_string("-") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .match_string("0")
                                                    .or_else(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                self::r#ASCII_NONZERO_DIGIT(state)
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| {
                                                                        state
                                                                            .sequence(|state| {
                                                                                state
                                                                                    .optional(|state| {
                                                                                        self::r#ASCII_DIGIT(state)
                                                                                            .and_then(|state| {
                                                                                                state
                                                                                                    .repeat(|state| {
                                                                                                        state
                                                                                                            .sequence(|state| {
                                                                                                                super::hidden::skip(state)
                                                                                                                    .and_then(|state| { self::r#ASCII_DIGIT(state) })
                                                                                                            })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#string(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::CompoundAtomic,
                            |state| {
                                state
                                    .rule(
                                        Rule::r#string,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    state
                                                        .match_string("\"\"\"")
                                                        .and_then(|state| { self::r#block_string_content(state) })
                                                        .and_then(|state| { state.match_string("\"\"\"") })
                                                })
                                                .or_else(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .match_string("\"")
                                                                .and_then(|state| { self::r#string_content(state) })
                                                                .and_then(|state| { state.match_string("\"") })
                                                        })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#block_string_content(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#block_string_content,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            state
                                                .repeat(|state| { self::r#block_string_character(state) })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#block_string_character(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#block_string_character,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .lookahead(
                                                false,
                                                |state| {
                                                    state
                                                        .match_string("\"\"\"")
                                                        .or_else(|state| { state.match_string("\\\"\"\"") })
                                                },
                                            )
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ANY(state) })
                                    })
                                    .or_else(|state| { state.match_string("\\\"\"\"") })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#string_content(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#string_content,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            state.repeat(|state| { self::r#string_character(state) })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#string_character(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#string_character,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .lookahead(
                                                false,
                                                |state| {
                                                    state
                                                        .match_string("\"")
                                                        .or_else(|state| { state.match_string("\\") })
                                                        .or_else(|state| { self::r#line_terminator(state) })
                                                },
                                            )
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ANY(state) })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                state
                                                    .match_string("\\")
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        state
                                                            .match_string("\"")
                                                            .or_else(|state| { state.match_string("\\") })
                                                            .or_else(|state| { state.match_string("/") })
                                                            .or_else(|state| { state.match_string("b") })
                                                            .or_else(|state| { state.match_string("f") })
                                                            .or_else(|state| { state.match_string("n") })
                                                            .or_else(|state| { state.match_string("r") })
                                                            .or_else(|state| { state.match_string("t") })
                                                    })
                                            })
                                    })
                                    .or_else(|state| {
                                        state
                                            .sequence(|state| {
                                                state
                                                    .match_string("\\u")
                                                    .and_then(|state| { super::hidden::skip(state) })
                                                    .and_then(|state| {
                                                        self::r#unicode_scalar_value_hex(state)
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#unicode_scalar_value_hex(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#unicode_scalar_value_hex,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .lookahead(
                                                false,
                                                |state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .match_insensitive("d")
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .match_range('8'..'9')
                                                                        .or_else(|state| { state.match_range('a'..'f') })
                                                                        .or_else(|state| { state.match_range('A'..'F') })
                                                                })
                                                        })
                                                },
                                            )
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ASCII_HEX_DIGIT(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ASCII_HEX_DIGIT(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ASCII_HEX_DIGIT(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#ASCII_HEX_DIGIT(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#boolean(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#boolean,
                            |state| {
                                state
                                    .match_string("true")
                                    .or_else(|state| { state.match_string("false") })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#null(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.rule(Rule::r#null, |state| { state.match_string("null") })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#enum_value(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .atomic(
                            ::pest::Atomicity::CompoundAtomic,
                            |state| {
                                state
                                    .rule(
                                        Rule::r#enum_value,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    state
                                                        .lookahead(
                                                            false,
                                                            |state| {
                                                                self::r#boolean(state)
                                                                    .or_else(|state| { self::r#null(state) })
                                                            },
                                                        )
                                                        .and_then(|state| { self::r#name(state) })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#const_list(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#const_list,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("[")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#const_value(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#const_value(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("]") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#list(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#list,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("[")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#value(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#value(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("]") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#const_object(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#const_object,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("{")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#const_object_field(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#const_object_field(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("}") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#object(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#object,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("{")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#object_field(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#object_field(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string("}") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#const_object_field(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#const_object_field,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#name(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#const_value(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#object_field(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#object_field,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#name(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#value(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#const_directives(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#const_directives,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#const_directive(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#const_directive(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#const_directive(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#directives(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#directives,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#directive(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .optional(|state| {
                                                                self::r#directive(state)
                                                                    .and_then(|state| {
                                                                        state
                                                                            .repeat(|state| {
                                                                                state
                                                                                    .sequence(|state| {
                                                                                        super::hidden::skip(state)
                                                                                            .and_then(|state| { self::r#directive(state) })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#const_directive(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#const_directive,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("@")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#const_arguments(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#directive(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#directive,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("@")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#name(state) })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state.optional(|state| { self::r#arguments(state) })
                                            })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#const_arguments(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#const_arguments,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("(")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#const_argument(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#const_argument(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#const_argument(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(")") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#arguments(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#arguments,
                            |state| {
                                state
                                    .sequence(|state| {
                                        state
                                            .match_string("(")
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| {
                                                state
                                                    .sequence(|state| {
                                                        self::r#argument(state)
                                                            .and_then(|state| { super::hidden::skip(state) })
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        state
                                                                            .optional(|state| {
                                                                                self::r#argument(state)
                                                                                    .and_then(|state| {
                                                                                        state
                                                                                            .repeat(|state| {
                                                                                                state
                                                                                                    .sequence(|state| {
                                                                                                        super::hidden::skip(state)
                                                                                                            .and_then(|state| { self::r#argument(state) })
                                                                                                    })
                                                                                            })
                                                                                    })
                                                                            })
                                                                    })
                                                            })
                                                    })
                                            })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(")") })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#const_argument(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#const_argument,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#name(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#const_value(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#argument(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#argument,
                            |state| {
                                state
                                    .sequence(|state| {
                                        self::r#name(state)
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { state.match_string(":") })
                                            .and_then(|state| { super::hidden::skip(state) })
                                            .and_then(|state| { self::r#value(state) })
                                    })
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#name_start(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#name_start,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            self::r#ASCII_ALPHA(state)
                                                .or_else(|state| { state.match_string("_") })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn r#name(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .rule(
                            Rule::r#name,
                            |state| {
                                state
                                    .atomic(
                                        ::pest::Atomicity::Atomic,
                                        |state| {
                                            state
                                                .sequence(|state| {
                                                    self::r#name_start(state)
                                                        .and_then(|state| {
                                                            state
                                                                .repeat(|state| {
                                                                    self::r#ASCII_ALPHA(state)
                                                                        .or_else(|state| { self::r#ASCII_DIGIT(state) })
                                                                        .or_else(|state| { state.match_string("_") })
                                                                })
                                                        })
                                                })
                                        },
                                    )
                            },
                        )
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .match_range('0'..'9')
                        .or_else(|state| state.match_range('a'..'f'))
                        .or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA(
                    state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                ) -> ::pest::ParseResult<
                    ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                > {
                    state
                        .match_range('a'..'z')
                        .or_else(|state| state.match_range('A'..'Z'))
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(
            input,
            |state| {
                match rule {
                    Rule::r#WHITESPACE => rules::r#WHITESPACE(state),
                    Rule::r#COMMENT => rules::r#COMMENT(state),
                    Rule::r#line_terminator => rules::r#line_terminator(state),
                    Rule::r#executable_document => rules::r#executable_document(state),
                    Rule::r#executable_definition => {
                        rules::r#executable_definition(state)
                    }
                    Rule::r#operation_definition => rules::r#operation_definition(state),
                    Rule::r#named_operation_definition => {
                        rules::r#named_operation_definition(state)
                    }
                    Rule::r#variable_definitions => rules::r#variable_definitions(state),
                    Rule::r#variable_definition => rules::r#variable_definition(state),
                    Rule::r#selection_set => rules::r#selection_set(state),
                    Rule::r#selection => rules::r#selection(state),
                    Rule::r#field => rules::r#field(state),
                    Rule::r#alias => rules::r#alias(state),
                    Rule::r#fragment_spread => rules::r#fragment_spread(state),
                    Rule::r#inline_fragment => rules::r#inline_fragment(state),
                    Rule::r#fragment_definition => rules::r#fragment_definition(state),
                    Rule::r#type_condition => rules::r#type_condition(state),
                    Rule::r#service_document => rules::r#service_document(state),
                    Rule::r#type_system_definition => {
                        rules::r#type_system_definition(state)
                    }
                    Rule::r#schema_definition => rules::r#schema_definition(state),
                    Rule::r#operation_type_definition => {
                        rules::r#operation_type_definition(state)
                    }
                    Rule::r#type_definition => rules::r#type_definition(state),
                    Rule::r#scalar_type => rules::r#scalar_type(state),
                    Rule::r#object_type => rules::r#object_type(state),
                    Rule::r#implements_interfaces => {
                        rules::r#implements_interfaces(state)
                    }
                    Rule::r#interface_type => rules::r#interface_type(state),
                    Rule::r#fields_definition => rules::r#fields_definition(state),
                    Rule::r#field_definition => rules::r#field_definition(state),
                    Rule::r#union_type => rules::r#union_type(state),
                    Rule::r#union_member_types => rules::r#union_member_types(state),
                    Rule::r#enum_type => rules::r#enum_type(state),
                    Rule::r#enum_values => rules::r#enum_values(state),
                    Rule::r#enum_value_definition => {
                        rules::r#enum_value_definition(state)
                    }
                    Rule::r#input_object_type => rules::r#input_object_type(state),
                    Rule::r#input_fields_definition => {
                        rules::r#input_fields_definition(state)
                    }
                    Rule::r#extend => rules::r#extend(state),
                    Rule::r#directive_definition => rules::r#directive_definition(state),
                    Rule::r#directive_locations => rules::r#directive_locations(state),
                    Rule::r#directive_location => rules::r#directive_location(state),
                    Rule::r#arguments_definition => rules::r#arguments_definition(state),
                    Rule::r#input_value_definition => {
                        rules::r#input_value_definition(state)
                    }
                    Rule::r#operation_type => rules::r#operation_type(state),
                    Rule::r#default_value => rules::r#default_value(state),
                    Rule::r#type_ => rules::r#type_(state),
                    Rule::r#const_value => rules::r#const_value(state),
                    Rule::r#value => rules::r#value(state),
                    Rule::r#variable => rules::r#variable(state),
                    Rule::r#number => rules::r#number(state),
                    Rule::r#float => rules::r#float(state),
                    Rule::r#fractional => rules::r#fractional(state),
                    Rule::r#exponent => rules::r#exponent(state),
                    Rule::r#int => rules::r#int(state),
                    Rule::r#string => rules::r#string(state),
                    Rule::r#block_string_content => rules::r#block_string_content(state),
                    Rule::r#block_string_character => {
                        rules::r#block_string_character(state)
                    }
                    Rule::r#string_content => rules::r#string_content(state),
                    Rule::r#string_character => rules::r#string_character(state),
                    Rule::r#unicode_scalar_value_hex => {
                        rules::r#unicode_scalar_value_hex(state)
                    }
                    Rule::r#boolean => rules::r#boolean(state),
                    Rule::r#null => rules::r#null(state),
                    Rule::r#enum_value => rules::r#enum_value(state),
                    Rule::r#const_list => rules::r#const_list(state),
                    Rule::r#list => rules::r#list(state),
                    Rule::r#const_object => rules::r#const_object(state),
                    Rule::r#object => rules::r#object(state),
                    Rule::r#const_object_field => rules::r#const_object_field(state),
                    Rule::r#object_field => rules::r#object_field(state),
                    Rule::r#const_directives => rules::r#const_directives(state),
                    Rule::r#directives => rules::r#directives(state),
                    Rule::r#const_directive => rules::r#const_directive(state),
                    Rule::r#directive => rules::r#directive(state),
                    Rule::r#const_arguments => rules::r#const_arguments(state),
                    Rule::r#arguments => rules::r#arguments(state),
                    Rule::r#const_argument => rules::r#const_argument(state),
                    Rule::r#argument => rules::r#argument(state),
                    Rule::r#name_start => rules::r#name_start(state),
                    Rule::r#name => rules::r#name(state),
                    Rule::EOI => rules::EOI(state),
                }
            },
        )
    }
}