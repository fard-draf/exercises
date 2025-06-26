use crate::domain::User;
use crate::error::*;
use std::collections::HashMap;

use crate::{
    domain::{AnalysisReport, LogLevel, ParsedLogEntry},
    error::*,
    parser::parse_log,
};

pub fn analyze(log: ParsedLogEntry) -> Result<HashMap<User, AnalysisReport>> {}
