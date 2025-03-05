/*
 * src/util/mode.rs
 *
 * This file is part of rnbook.
 *
 * rnbook is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * rnbook is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with rnbook. If not, see <https://www.gnu.org/licenses/>.
 */

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenMode {
    EDIT,
    READ,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeT {
    BROWSE,
    OPEN(OpenMode),
}

impl fmt::Display for ModeT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModeT::BROWSE => write!(f, "BROWSE"),
            ModeT::OPEN(open_mode) => write!(f, "OPEN({})", open_mode),
        }
    }
}

impl fmt::Display for OpenMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenMode::EDIT => write!(f, "EDIT"),
            OpenMode::READ => write!(f, "READ"),
        }
    }
}
