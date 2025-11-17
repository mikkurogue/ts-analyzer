#[derive(Debug, Clone)]
pub struct TsError {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub code: CommonErrors,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum CommonErrors {
    TypeMismatch,
    InlineTypeMismatch,
    MissingParameters,
    NoImplicitAny,
    PropertyMissingInType,
    UnintentionalComparison,
    PropertyDoesNotExist,
    ObjectIsPossiblyUndefined,
    ObjectIsPossiblyNull,
    ObjectIsUnknown,
    DirectCastPotentiallyMistaken,
    SpreadArgumentMustBeTupleType,
    RightSideArithmeticMustBeNumber,
    IncompatibleOverload,
    InvalidShadowInScope,
    NonExistentModuleImport,
    ReadonlyPropertyAssignment,
    IncorrectInterfaceImplementation,
    Unsupported(String),
}

impl std::fmt::Display for CommonErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonErrors::TypeMismatch => write!(f, "TS2322"),
            CommonErrors::InlineTypeMismatch => write!(f, "TS2345"),
            CommonErrors::MissingParameters => write!(f, "TS2554"),
            CommonErrors::NoImplicitAny => write!(f, "TS7006"),
            CommonErrors::PropertyMissingInType => write!(f, "TS2741"),
            CommonErrors::UnintentionalComparison => write!(f, "TS2367"),
            CommonErrors::PropertyDoesNotExist => write!(f, "TS2339"),
            CommonErrors::ObjectIsPossiblyUndefined => write!(f, "TS2532"),
            CommonErrors::ObjectIsPossiblyNull => write!(f, "TS2531"),
            CommonErrors::ObjectIsUnknown => write!(f, "TS18046"),
            CommonErrors::DirectCastPotentiallyMistaken => write!(f, "TS2352"),
            CommonErrors::SpreadArgumentMustBeTupleType => write!(f, "TS2556"),
            CommonErrors::RightSideArithmeticMustBeNumber => write!(f, "TS2363"),
            CommonErrors::IncompatibleOverload => write!(f, "TS2394"),
            CommonErrors::InvalidShadowInScope => write!(f, "TS2451"),
            CommonErrors::NonExistentModuleImport => write!(f, "TS2307"),
            CommonErrors::ReadonlyPropertyAssignment => write!(f, "TS2540"),
            CommonErrors::IncorrectInterfaceImplementation => write!(f, "TS2420"),
            CommonErrors::Unsupported(code) => write!(f, "{}", code),
        }
    }
}

impl CommonErrors {
    pub fn from_code(code: &str) -> CommonErrors {
        match code {
            "TS2322" => CommonErrors::TypeMismatch,
            "TS2345" => CommonErrors::InlineTypeMismatch,
            "TS2554" => CommonErrors::MissingParameters,
            "TS7006" | "TS7044" => CommonErrors::NoImplicitAny,
            "TS2741" => CommonErrors::PropertyMissingInType,
            "TS2367" => CommonErrors::UnintentionalComparison,
            "TS18046" => CommonErrors::ObjectIsUnknown,
            "TS2339" => CommonErrors::PropertyDoesNotExist,
            "TS2532" | "TS18048" => CommonErrors::ObjectIsPossiblyUndefined,
            "TS2531" | "TS18047" => CommonErrors::ObjectIsPossiblyNull,
            "TS2352" => CommonErrors::DirectCastPotentiallyMistaken,
            "TS2556" => CommonErrors::SpreadArgumentMustBeTupleType,
            "TS2363" => CommonErrors::RightSideArithmeticMustBeNumber,
            "TS2394" => CommonErrors::IncompatibleOverload,
            "TS2451" => CommonErrors::InvalidShadowInScope,
            "TS2307" => CommonErrors::NonExistentModuleImport,
            "TS2540" => CommonErrors::ReadonlyPropertyAssignment,
            "TS2420" => CommonErrors::IncorrectInterfaceImplementation,
            other => CommonErrors::Unsupported(other.to_string()),
        }
    }
}

pub fn parse(line: &str) -> Option<TsError> {
    let (file, rest) = line.split_once('(')?;
    let (coords, rest) = rest.split_once("): error ")?;
    let (line_s, col_s) = coords.split_once(',')?;
    let (code, msg) = rest.split_once(": ")?;

    Some(TsError {
        file: file.to_string(),
        line: usize::from_str_radix(line_s, 10).ok()?,
        column: usize::from_str_radix(col_s, 10).ok()?,
        code: CommonErrors::from_code(code),
        message: msg.to_string(),
    })
}
