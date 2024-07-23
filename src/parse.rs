// SPDX-FileCopyrightText: 2024 University of Rochester
//
// SPDX-License-Identifier: MIT

use naga::{
    front::wgsl::parse_str,
    valid::{Capabilities, ValidationFlags, Validator},
};

/// Parse the given WGSL shader code and return the parsed module
///
/// # Arguments
///
/// * `shader` - The WGSL shader code to parse
pub(crate) fn parse_wgsl(shader: &str) -> Result<naga::Module, String> {
    let module = parse_str(shader).map_err(|e| e.to_string())?;

    // Validate the parsed WGSL module
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
    validator.validate(&module).map_err(|e| e.to_string())?;

    Ok(module)
}

/// Helper method that returns an Iterator over all of the functions (including those inside entry points) for the module.
///
/// # Arguments
///
/// * `module` - The parsed WGSL module.
/// * `filterer` - A closure that takes an entry point and returns an iterator of expressions.
#[inline]
fn iter_module_functions(module: &naga::Module) -> impl Iterator<Item = &naga::Function> {
    // Chain the iters for entry points and functions
    module
        .entry_points
        .iter()
        .map(|f| &f.function)
        .chain(module.functions.iter().map(|f| f.1))
}
#[allow(dead_code)]
/// Return an iterator yielding the condition of all `If` statements from the given module
pub(crate) fn iter_if_conditions(
    module: &naga::Module,
) -> impl Iterator<Item = (&naga::Function, naga::Handle<naga::Expression>)> {
    iter_module_functions(module).flat_map(|fun: &naga::Function| {
        fun.body.iter().filter_map(move |s| match s {
            naga::Statement::If { condition, .. } => Some((fun, *condition)),
            _ => None,
        })
    })
}

/// Return an iterator yielding the condition of all `Loop` statements from the given module
pub(crate) fn iter_loop_conditions(
    module: &naga::Module,
) -> impl Iterator<Item = (&naga::Function, naga::Handle<naga::Expression>)> {
    iter_module_functions(module).flat_map(|f| {
        f.body.iter().filter_map(move |s| match s {
            naga::Statement::Loop {
                break_if: Some(condition),
                ..
            } => Some((f, *condition)),
            _ => None,
        })
    })
}

/// Yield expressions of a specified kind from the entry points in the module.
///
/// # Arguments
///
/// * `module` - The parsed wgsl module.
///
/// # Returns
///
/// An iterator yielding tuples of the entry point contianing the expression and the expression itself.
///
pub fn iter_access_expr<'a>(
    module: &'a naga::Module,
) -> impl Iterator<Item = (&naga::Function, naga::Handle<naga::Expression>)> {
    // Local refs to module's types and vars
    let mod_types = &module.types;
    let mod_gvars = &module.global_variables;
    let is_not_struct = |ty: naga::Handle<naga::Type>| -> bool {
        !matches!(mod_types[ty].inner, naga::TypeInner::Struct { .. })
    };
    iter_module_functions(module).flat_map(move |f| {
        f.expressions.iter().filter_map(move |e| match e.1 {
            naga::Expression::Access { .. } => Some((f, e.0)),
            naga::Expression::AccessIndex { base, .. } => {
                let expr2 = &f.expressions[*base];
                match expr2 {
                    naga::Expression::GlobalVariable(x) if is_not_struct(mod_gvars[*x].ty) => {
                        Some((f, e.0))
                    }
                    naga::Expression::LocalVariable(x)
                        if is_not_struct(f.local_variables[*x].ty) =>
                    {
                        Some((f, e.0))
                    }
                    _ => None,
                }
            }
            _ => None,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::vec;

    static WGSL_SHADER_1: &'static str = r#"
    @group(0) @binding(0) var<storage, read> A : array<f32>;
    @group(0) @binding(1) var<storage, read_write> B : array<f32>;

    struct PODArgs {
        n: i32,
        packGridDimX: u32
    }
    @group(0) @binding(2) var<uniform> podArgs : PODArgs;

    @compute @workgroup_size(32, 1, 1)
    fn main_kernel(
    @builtin(workgroup_id) blockIdx : vec3<u32>,
    @builtin(num_workgroups) gridDim : vec3<u32>,
    @builtin(local_invocation_id) threadIdx : vec3<u32>
    ) {
        if (blockIdx.z * gridDim.x + blockIdx.x > podArgs.packGridDimX) { return; }
        let v__1 : i32 = i32(blockIdx.z * gridDim.x + blockIdx.x);
        if (((v__1 * 32i) + i32(threadIdx.x)) < podArgs.n) {
            B[((v__1 * 32i) + i32(threadIdx.x))] = log(abs((A[((v__1 * 32i) + i32(threadIdx.x))] + 1.000000e+00f)));
        }
    }
    "#;

    static WGSL_SHADER_2: &'static str = r#"
    @group(0) @binding(0) var<storage, read> A : array<f32>;
    @group(0) @binding(1) var<storage, read_write> B : array<f32>;

    struct PODArgs {
        n: i32,
        packGridDimX: u32
    }
    @group(0) @binding(2) var<uniform> podArgs : PODArgs;

    @compute @workgroup_size(32, 1, 1)
    fn main_kernel(
    @builtin(workgroup_id) blockIdx : vec3<u32>,
    @builtin(num_workgroups) gridDim : vec3<u32>,
    @builtin(local_invocation_id) threadIdx : vec3<u32>
    ) {
        let v__2: f32 = B[1+2];
        if (blockIdx.z * gridDim.x + blockIdx.x > podArgs.packGridDimX) { return; }
        let v__1 : i32 = i32(blockIdx.z * gridDim.x + blockIdx.x);
        if (((v__1 * 32i) + i32(threadIdx.x)) < podArgs.n) {
            B[((v__1 * 32i) + i32(threadIdx.x))] = log(abs((A[((v__1 * 32i) + i32(threadIdx.x))] + 1.000000e+00f)));
        }
    }
    "#;

    #[rstest]
    #[case(WGSL_SHADER_1,
           vec!["blockIdx.z * gridDim.x + blockIdx.x > podArgs.packGridDimX",
                "((v__1 * 32i) + i32(threadIdx.x)) < podArgs.n"])]
    fn test_iter_if_conditions(#[case] wgsl_shader: &'static str, #[case] expected: Vec<&str>) {
        let module = parse_wgsl(wgsl_shader).unwrap();

        assert_eq!(
            &expected[..],
            iter_if_conditions(&module)
                .map(|c| write_expression(wgsl_shader, &c.0, c.1))
                .collect::<Vec<&str>>()
        );
    }

    #[rstest]
    #[case(WGSL_SHADER_1, vec!["B[((v__1 * 32i) + i32(threadIdx.x))]", "A[((v__1 * 32i) + i32(threadIdx.x))]"])]
    #[case(WGSL_SHADER_2, vec!["B[1+2]", "B[((v__1 * 32i) + i32(threadIdx.x))]", "A[((v__1 * 32i) + i32(threadIdx.x))]"])]
    fn test_iter_access(#[case] wgsl_shader: &'static str, #[case] expected: Vec<&str>) {
        let module = parse_wgsl(wgsl_shader).unwrap();
        let result = iter_access_expr(&module)
            .map(|c| write_expression(wgsl_shader, &c.0, &c.1))
            .collect::<Vec<&str>>();
        assert_eq!(&expected[..], result);
    }
}
