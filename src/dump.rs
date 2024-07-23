// SPDX-FileCopyrightText: 2024 University of Rochester
//
// SPDX-License-Identifier: MIT

/// Return the string representation of the given expression from the source code
///
/// # Arguments
///
/// * `source` - The source code of the shader
/// * `function` - The function containing the expression
/// * `handle` - The handle of the expression
///
/// # Returns
///
/// The string representation of the expression
pub fn write_expression<'a>(
    source: &'a str,
    function: &naga::Function,
    handle: &naga::Handle<naga::Expression>,
) -> &'a str {
    &source[function.expressions.get_span(*handle)]
}
