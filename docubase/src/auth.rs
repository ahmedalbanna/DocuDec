use crate::models::{Claims, User};
use crate::utils::{generate_jwt, hash_password};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;
