use crate::parser::{CommonErrors, TsError};
use crate::tokenizer::Token;
use colored::*;

pub trait Suggest {
    fn build(err: &TsError, tokens: &[Token]) -> Option<Self>
    where
        Self: Sized;
}

pub struct Suggestion {
    pub suggestions: Vec<String>,
    pub help: Option<String>,
}

trait SuggestionHandler {
    fn handle(&self, err: &TsError, tokens: &[Token]) -> Option<Suggestion>;
}

struct TypeMismatchHandler;
impl SuggestionHandler for TypeMismatchHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        Some(Suggestion {
            suggestions: vec![type_mismatch_2322(err)?],
            help: Some(
                "Ensure that the types are compatible or perform an explicit conversion."
                    .to_string(),
            ),
        })
    }
}

struct InlineTypeMismatchHandler;
impl SuggestionHandler for InlineTypeMismatchHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let suggestions = inline_type_mismatch_2345(err);
        Some(Suggestion {
            suggestions: suggestions.unwrap_or_default(),
            help: Some(
                "Check the function arguments to ensure they match the expected parameter types."
                    .to_string(),
            ),
        })
    }
}

struct MissingParametersHandler;
impl SuggestionHandler for MissingParametersHandler {
    fn handle(&self, err: &TsError, tokens: &[Token]) -> Option<Suggestion> {
        let mut fn_name = err
            .message
            .split('\'')
            .nth(1)
            .unwrap_or("function")
            .to_string();

        for token in tokens {
            if token.line == err.line
                && (err.column - 1) >= token.column
                && (err.column - 1) < token.column + token.raw.chars().count()
            {
                fn_name = token.raw.clone();
                break;
            }
        }

        Some(Suggestion {
            suggestions: vec![format!(
                "Check if all required arguments are provided when invoking {}",
                fn_name.red().bold()
            )],
            help: Some(format!(
                "Function `{}` is missing 1 or more arguments.",
                fn_name.red().bold()
            )),
        })
    }
}

struct NoImplicitAnyHandler;
impl SuggestionHandler for NoImplicitAnyHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let param_name = err.message.split('\'').nth(1).unwrap_or("parameter");

        Some(Suggestion {
            suggestions: vec![format!("{} is implicitly `any`.", param_name.red().bold())],
            help: Some(
                "Consider adding type annotations to avoid implicit 'any' types.".to_string(),
            ),
        })
    }
}

struct PropertyMissingInTypeHandler;
impl SuggestionHandler for PropertyMissingInTypeHandler {
    fn handle(&self, err: &TsError, tokens: &[Token]) -> Option<Suggestion> {
        if let Some(type_name) = parse_property_missing_error(&err.message) {
            let mut var_name: String = String::new();
            for token in tokens {
                if token.line == err.line
                    && (err.column - 1) >= token.column
                    && (err.column - 1) < token.column + token.raw.chars().count()
                {
                    var_name = token.raw.clone();
                    break;
                }
            }

            Some(Suggestion {
                suggestions: vec![format!(
                    "Verify that `{}` matches the annotated type `{}`.",
                    var_name.red().bold().italic(),
                    type_name.red().bold()
                )],
                help: Some(format!(
                    "Ensure that `{}` has all required properties defined in the type `{}`.",
                    var_name.red().bold().italic(),
                    type_name.red().bold()
                )),
            })
        } else {
            Some(Suggestion {
                suggestions: vec![
                    "Verify that the object structure includes all required members of the specified type."
                        .to_string()
                ],
                help: Some(
                    "Ensure the object has all required properties defined in the type."
                        .to_string(),
                ),
            })
        }
    }
}

struct UnintentionalComparisonHandler;
impl SuggestionHandler for UnintentionalComparisonHandler {
    fn handle(&self, _err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        Some(Suggestion {
            suggestions: vec![
                "Impossible to compare as left side value is narrowed to a single value."
                    .to_string(),
            ],
            help: Some("Review the comparison logic to ensure it makes sense.".to_string()),
        })
    }
}

struct PropertyDoesNotExistHandler;
impl SuggestionHandler for PropertyDoesNotExistHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let property_name = err.message.split('\'').nth(1).unwrap_or("property");
        let type_name = err.message.split('\'').nth(3).unwrap_or("type");

        Some(Suggestion {
            suggestions: vec![format!(
                "Property `{}` is not found on type `{}`.",
                property_name.red().bold(),
                type_name.red().bold()
            )],
            help: Some(
                "Ensure the property exists on the type or adjust your code to avoid accessing it."
                    .to_string(),
            ),
        })
    }
}

struct ObjectIsPossiblyUndefinedHandler;
impl SuggestionHandler for ObjectIsPossiblyUndefinedHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let possible_undefined_var = err
            .message
            .split('\'')
            .nth(1)
            .unwrap_or("object")
            .to_string();

        Some(Suggestion {
            suggestions: vec![format!(
                "{} may be `undefined` here.",
                possible_undefined_var.red().bold()
            )],
            help: Some(format!(
                "Consider optional chaining or an explicit check before attempting to access `{}`",
                possible_undefined_var.red().bold()
            )),
        })
    }
}

struct DirectCastPotentiallyMistakenHandler;
impl SuggestionHandler for DirectCastPotentiallyMistakenHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let cast_from_type = err.message.split('\'').nth(1).unwrap_or("type");
        let cast_to_type = err.message.split('\'').nth(3).unwrap_or("type");

        Some(Suggestion {
            suggestions: vec![format!(
                "Directly casting from `{}` to `{}` can be unsafe or mistaken, as both types do not overlap sufficiently.",
                cast_from_type.yellow().bold(),
                cast_to_type.yellow().bold()
            )],
            help: Some(format!(
                "Consider using type guards or intermediate conversions to ensure type safety when casting from `{}` to `{}`, only intermediately cast `as unknown` if this is desired.",
                cast_from_type.yellow().bold(),
                cast_to_type.yellow().bold()
            )),
        })
    }
}

struct SpreadArgumentMustBeTupleTypeHandler;
impl SuggestionHandler for SpreadArgumentMustBeTupleTypeHandler {
    fn handle(&self, _err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        Some(Suggestion {
            suggestions: vec![
                "The argument being spread must be a tuple type or a `spreadable` type."
                    .to_string()
            ],
            help: Some(
                "Ensure that the argument being spread is a tuple type compatible with the function's parameter type."
                    .to_string(),
            ),
        })
    }
}

struct RightSideArithmeticMustBeEnumberableHandler;
impl SuggestionHandler for RightSideArithmeticMustBeEnumberableHandler {
    fn handle(&self, _err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        Some(Suggestion {
            suggestions: vec![
                "The right-hand side of any arithmetic operation must be a number or enumerable."
                    .to_string()
            ],
            help: Some(
                "Ensure that the value on the right side of the arithmetic operator is of type `number`, `bigint` or an enum member."
                    .to_string(),
            ),
        })
    }
}

struct LeftSideArithmeticMustBeEnumberableHandler;
impl SuggestionHandler for LeftSideArithmeticMustBeEnumberableHandler {
    fn handle(&self, _err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        Some(Suggestion {
            suggestions: vec![
                "The left-hand side of any arithmetic operation must be a number or enumerable."
                    .to_string()
            ],
            help: Some(
                "Ensure that the value on the left side of the arithmetic operator is of type `number`, `bigint` or an enum member."
                    .to_string(),
            ),
        })
    }
}

struct IncompatibleOverloadHandler;
impl SuggestionHandler for IncompatibleOverloadHandler {
    fn handle(&self, _err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        Some(Suggestion {
            suggestions: vec![
                "The provided arguments do not match any overload of the function."
                    .to_string()
            ],
            help: Some(
                "Check the function overloads and ensure that this signature adheres to the parent signature."
                    .to_string(),
            ),
        })
    }
}

struct InvalidShadowInScopeHandler;
impl SuggestionHandler for InvalidShadowInScopeHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let var_name = err.message.split('\'').nth(1).unwrap_or("variable");

        Some(Suggestion {
            suggestions: vec![format!(
                "Declared variable `{}` can not shadow another variable in this scope.",
                var_name.red().bold()
            )],
            help: Some(format!(
                "Consider renaming the invalid shadowed variable `{}`.",
                var_name.red().bold()
            )),
        })
    }
}

struct NonExistentModuleImportHandler;
impl SuggestionHandler for NonExistentModuleImportHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let module_name = err.message.split('\'').nth(1).unwrap_or("module");

        Some(Suggestion {
            suggestions: vec![format!(
                "Module `{}` does not exist.",
                module_name.red().bold()
            )],
            help: Some(format!(
                "Ensure that the module `{}` is installed and the import path is correct.",
                module_name.red().bold(),
            )),
        })
    }
}

struct ReadonlyPropertyAssignmentHandler;
impl SuggestionHandler for ReadonlyPropertyAssignmentHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let property_name = err.message.split('\'').nth(1).unwrap_or("property");

        Some(Suggestion {
            suggestions: vec![format!(
                "Property `{}` is readonly and thus can not be re-assigned.",
                property_name.red().bold()
            )],
            help: Some(format!(
                "Consider removing the assignment to the read-only property `{}` or changing its declaration to be mutable.",
                property_name.red().bold()
            )),
        })
    }
}

struct IncorrectInterfaceImplementationHandler;
impl SuggestionHandler for IncorrectInterfaceImplementationHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let class_name = err.message.split('\'').nth(1).unwrap_or("class");
        let interface_name = err.message.split('\'').nth(3).unwrap_or("interface");
        let missing_property = err.message.split('\'').nth(5).unwrap_or("property");

        Some(Suggestion {
            suggestions: vec![format!(
                "Class `{}` does not implement `{}` from interface `{}`.",
                class_name.red().bold(),
                missing_property.red().bold(),
                interface_name.red().bold()
            )],
            help: Some(format!(
                "Ensure that `{}` provides all required properties and methods defined in the interface `{}`.",
                class_name.red().bold(),
                interface_name.red().bold()
            )),
        })
    }
}

struct PropertyInClassNotAssignableToBaseHandler;
impl SuggestionHandler for PropertyInClassNotAssignableToBaseHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let property = err.message.split('\'').nth(1).unwrap_or("property");
        let impl_type = err.message.split('\'').nth(3).unwrap_or("type");
        let base_type = err.message.split('\'').nth(5).unwrap_or("base type");
        let property_impl_type = err.message.split('\'').nth(7).unwrap_or("type");
        let property_base_type = err.message.split('\'').nth(9).unwrap_or("base type");

        Some(Suggestion {
            suggestions: vec![
                format!(
                    "Property `{}` in class `{}` is not assignable to the same property in base class `{}`.",
                    property.red().bold(),
                    impl_type.red().bold(),
                    base_type.red().bold()
                ),
                format!(
                    "Property `{}` is implemented as type `{}` but defined as `{}`.",
                    property.red().bold(),
                    property_impl_type.red().bold(),
                    property_base_type.green().bold()
                ),
            ],
            help: Some(format!(
                "Ensure that the type of property `{}` in class `{}` is compatible with the type defined in base class `{}`.",
                property.red().bold(),
                impl_type.red().bold(),
                base_type.red().bold()
            )),
        })
    }
}

struct CannotFindIdentifierHandler;
impl SuggestionHandler for CannotFindIdentifierHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let identifier = err.message.split('\'').nth(1).unwrap_or("identifier");

        Some(Suggestion {
            suggestions: vec![format!(
                "Identifier `{}` cannot be found in the current scope.",
                identifier.red().bold()
            )],
            help: Some(format!(
                "Ensure that `{}` is declared and accessible in the current scope or remove this reference.",
                identifier.red().bold()
            )),
        })
    }
}

struct MissingReturnValueHandler;
impl SuggestionHandler for MissingReturnValueHandler {
    fn handle(&self, _err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        Some(Suggestion {
            suggestions: vec![
                "A return value is missing where one is expected.".to_string()
            ],
            help: Some(
                "A function that declares a return type must return a value of that type on all branches."
                    .to_string(),
            ),
        })
    }
}

struct UncallableExpressionHandler;
impl SuggestionHandler for UncallableExpressionHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let expr = err.message.split('\'').nth(1).unwrap_or("expression");

        Some(Suggestion {
            suggestions: vec![format!(
                "Expression `{}` not can not be invoked or called.",
                expr.red().bold()
            )],
            help: Some(format!(
                "Ensure that `{}` is a function or has a callable signature before invoking it.",
                expr.red().bold()
            )),
        })
    }
}

struct InvalidIndexTypeHandler;
impl SuggestionHandler for InvalidIndexTypeHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let index_type = err.message.split('\'').nth(1).unwrap_or("type");

        Some(Suggestion {
            suggestions: vec![format!(
                "`{}` cannot be used as an index accessor.",
                index_type.red().bold()
            )],
            help: Some("Ensure that the index type is `number`, `string`, `symbole` or a compatible index type.".to_string()),
        })
    }
}

struct TypoPropertyOnTypeHandler;
impl SuggestionHandler for TypoPropertyOnTypeHandler {
    fn handle(&self, err: &TsError, _tokens: &[Token]) -> Option<Suggestion> {
        let property_name = err.message.split('\'').nth(1).unwrap_or("property");
        let type_name = err.message.split('\'').nth(3).unwrap_or("type");
        let suggested_property_name = err.message.split('\'').nth(5).unwrap_or("property");

        Some(Suggestion {
            suggestions: vec![format!(
                "Property `{}` does not exist on type `{}`. Try `{}` instead",
                property_name.red().bold(),
                type_name.yellow().bold(),
                suggested_property_name.green().bold()
            )],
            help: Some(format!(
                "Check for typos in the property name `{}` or ensure that it is defined on type `{}`.",
                property_name.red().bold(),
                type_name.red().bold()
            )),
        })
    }
}

impl Suggest for Suggestion {
    /// Build a suggestion and help text for the given TsError
    fn build(err: &TsError, tokens: &[Token]) -> Option<Self> {
        let handler: Box<dyn SuggestionHandler> = match err.code {
            CommonErrors::TypeMismatch => Box::new(TypeMismatchHandler),
            CommonErrors::InlineTypeMismatch => Box::new(InlineTypeMismatchHandler),
            CommonErrors::MissingParameters => Box::new(MissingParametersHandler),
            CommonErrors::NoImplicitAny => Box::new(NoImplicitAnyHandler),
            CommonErrors::PropertyMissingInType => Box::new(PropertyMissingInTypeHandler),
            CommonErrors::UnintentionalComparison => Box::new(UnintentionalComparisonHandler),
            CommonErrors::PropertyDoesNotExist => Box::new(PropertyDoesNotExistHandler),
            CommonErrors::ObjectIsPossiblyUndefined => Box::new(ObjectIsPossiblyUndefinedHandler),
            CommonErrors::DirectCastPotentiallyMistaken => {
                Box::new(DirectCastPotentiallyMistakenHandler)
            }
            CommonErrors::SpreadArgumentMustBeTupleType => {
                Box::new(SpreadArgumentMustBeTupleTypeHandler)
            }
            CommonErrors::RightSideArithmeticMustBeEnumberable => {
                Box::new(RightSideArithmeticMustBeEnumberableHandler)
            }
            CommonErrors::LeftSideArithmeticMustBeEnumberable => {
                Box::new(LeftSideArithmeticMustBeEnumberableHandler)
            }
            CommonErrors::IncompatibleOverload => Box::new(IncompatibleOverloadHandler),
            CommonErrors::InvalidShadowInScope => Box::new(InvalidShadowInScopeHandler),
            CommonErrors::NonExistentModuleImport => Box::new(NonExistentModuleImportHandler),
            CommonErrors::ReadonlyPropertyAssignment => Box::new(ReadonlyPropertyAssignmentHandler),
            CommonErrors::IncorrectInterfaceImplementation => {
                Box::new(IncorrectInterfaceImplementationHandler)
            }
            CommonErrors::PropertyInClassNotAssignableToBase => {
                Box::new(PropertyInClassNotAssignableToBaseHandler)
            }
            CommonErrors::CannotFindIdentifier => Box::new(CannotFindIdentifierHandler),
            CommonErrors::MissingReturnValue => Box::new(MissingReturnValueHandler),
            CommonErrors::UncallableExpression => Box::new(UncallableExpressionHandler),
            CommonErrors::InvalidIndexType => Box::new(InvalidIndexTypeHandler),
            CommonErrors::TypoPropertyOnType => Box::new(TypoPropertyOnTypeHandler),
            // TODO: figure out why both of these 2 are not parsing correctly
            CommonErrors::ObjectIsPossiblyNull => return None,
            CommonErrors::ObjectIsUnknown => return None,
            CommonErrors::Unsupported(_) => return None,
        };

        handler.handle(err, tokens)
    }
}

/// Suggestion helper for ts2322
fn type_mismatch_2322(err: &TsError) -> Option<String> {
    if let Some((from, to)) = parse_ts2322_error(&err.message) {
        Some(format!(
            "Try converting this value from `{}` to `{}`.",
            from.red().bold(),
            to.green().bold()
        ))
    } else {
        None
    }
}

/// Suggestion helper for ts2345
fn inline_type_mismatch_2345(err: &TsError) -> Option<Vec<String>> {
    if let Some(mismatches) = parse_ts2345_error(&err.message) {
        if mismatches.is_empty() {
            return None;
        }

        let lines: Vec<String> = mismatches
            .iter()
            .map(|(property, provided, expected)| {
                format!(
                    "Property `{}` is provided as `{}` but expects `{}`.",
                    property.red().bold(),
                    provided.red().bold(),
                    expected.green().bold()
                )
            })
            .collect();

        Some(lines)
    } else {
        None
    }
}

fn parse_ts2322_error(msg: &str) -> Option<(String, String)> {
    let mut chars = msg.chars().peekable();

    fn read_quoted<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> Option<String> {
        // Expect starting `'`
        if chars.next()? != '\'' {
            return None;
        }
        let mut out = String::new();
        while let Some(&c) = chars.peek() {
            chars.next();
            if c == '\'' {
                break;
            }
            out.push(c);
        }
        Some(out)
    }

    while let Some(_) = chars.next() {
        let mut lookahead = chars.clone();
        if lookahead.next()? == 'y'
            && lookahead.next()? == 'p'
            && lookahead.next()? == 'e'
            && lookahead.next()? == ' '
            && lookahead.next()? == '\''
        {
            for _ in 0..4 {
                chars.next();
            }
            let from = read_quoted(&mut chars)?;

            while let Some(c) = chars.next() {
                if c == '\'' {
                    let mut secondary = String::new();
                    for c2 in chars.by_ref() {
                        if c2 == '\'' {
                            break;
                        }
                        secondary.push(c2);
                    }
                    return Some((from, secondary));
                }
            }
        }
    }

    None
}

fn parse_property_missing_error(msg: &str) -> Option<String> {
    let type_marker = "type '";
    if let Some(start_index) = msg.rfind(type_marker) {
        let rest_of_msg = &msg[start_index + type_marker.len()..];
        if let Some(end_index) = rest_of_msg.find('\'') {
            return Some(rest_of_msg[..end_index].to_string());
        }
    }
    None
}

fn parse_ts2345_error(msg: &str) -> Option<Vec<(String, String, String)>> {
    let provided_obj = extract_object_type(msg, "Argument of type '")?;
    let expected_obj = extract_object_type(msg, "to parameter of type '")?;

    let provided_props = parse_object_properties(&provided_obj);
    let expected_props = parse_object_properties(&expected_obj);

    // Find all mismatched properties
    let mut mismatches = Vec::new();
    for (key, expected_type) in &expected_props {
        if let Some(provided_type) = provided_props.get(key)
            && provided_type != expected_type
        {
            mismatches.push((key.clone(), provided_type.clone(), expected_type.clone()));
        }
    }

    Some(mismatches)
}

fn extract_object_type(msg: &str, marker: &str) -> Option<String> {
    let start = msg.find(marker)? + marker.len();
    let rest = &msg[start..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

fn parse_object_properties(obj_type: &str) -> std::collections::HashMap<String, String> {
    let mut props = std::collections::HashMap::new();

    let obj_type = obj_type.trim();
    if !obj_type.starts_with('{') || !obj_type.ends_with('}') {
        return props;
    }

    let inner = &obj_type[1..obj_type.len() - 1];

    for prop in inner.split(';') {
        let prop = prop.trim();
        if prop.is_empty() {
            continue;
        }

        if let Some(colon_pos) = prop.find(':') {
            let key = prop[..colon_pos].trim().to_string();
            let value = prop[colon_pos + 1..].trim().to_string();
            props.insert(key, value);
        }
    }

    props
}
