// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::{Function, Input, Type, Variant};
use leo_span::Span;

use serde::{Deserialize, Serialize};

use crate::SymbolTable;

/// Metadata associated with the finalize block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizeData {
    /// The inputs to the finalize block.
    pub(crate) input: Vec<Input>,
    /// The output type of the finalize block.
    pub(crate) output_type: Type,
}

/// An entry for a function in the symbol table.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionSymbol {
    /// The index associated with the scope in the parent symbol table.
    pub(crate) id: usize,
    /// Whether the function is asynchronous or not.
    pub(crate) is_async: bool,
    /// The output type of the function.
    pub(crate) output_type: Type,
    /// Is this function a transition, inlined, or a regular function?.
    pub variant: Variant,
    /// The `Span` associated with the function.
    pub(crate) _span: Span,
    /// The inputs to the function.
    pub(crate) input: Vec<Input>,
}

impl SymbolTable {
    pub(crate) fn new_function_symbol(id: usize, func: &Function) -> FunctionSymbol {
        FunctionSymbol {
            id,
            is_async: func.is_async,
            output_type: func.output_type.clone(),
            variant: func.variant,
            _span: func.span,
            input: func.input.clone()
        }
    }
}
