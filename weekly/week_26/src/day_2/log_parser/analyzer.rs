use std::collections::HashMap;
use crate::domain::User;
use crate::error::*;

use crate::{
    domain::{LogLevel, ParsedLogEntry, AnalysisReport}, error::*, parser::parse_log
};
