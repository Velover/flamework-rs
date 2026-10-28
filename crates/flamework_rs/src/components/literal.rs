//! roblox-ts's lowering of an object literal (`transformObjectLiteralExpression`, `util/pointer.ts`), for the config
//! literal the TS rebuilt in its own field order, which `compile_object_literal` cannot emit: fields go in a table
//! built inline (`{...}`) until one needs statements, then moved into a local and filled by assignments.

use rbxts_luau::{AssignmentOperator, Expr, MapField, Stmt, create};
use rbxts_plugin::{Cx, LuauExpr};

/// `MapPointer`: the table, inline while it can be, then the local it moved into.
pub(crate) struct MapPointer {
    pub(crate) value: LuauExpr,
}

impl Default for MapPointer {
    fn default() -> Self {
        Self { value: create::map([]) }
    }
}

impl MapPointer {
    /// `disableMapInline`: `local _object = {...}`.
    fn disable_inline(&mut self, cx: &mut Cx<'_>) {
        if matches!(self.value, Expr::Map(_)) {
            let map = std::mem::replace(&mut self.value, Expr::None);
            self.value = cx.push_to_var(map, "object");
        }
    }

    /// `assignToMapPointer`: a field of the inline table, or `_object[left] = right`.
    fn assign(&mut self, cx: &mut Cx<'_>, left: LuauExpr, right: LuauExpr) {
        if let Expr::Map(fields) = &mut self.value {
            fields.push(MapField { index: left, value: right });
        } else {
            let target = create::computed_index(self.value.clone(), left);
            cx.prereq(create::assignment(target, AssignmentOperator::Assign, right));
        }
    }

    /// `transformPropertyAssignment`: a field whose key and value were compiled with the statements each needs.
    pub(crate) fn field(&mut self, cx: &mut Cx<'_>, key: (LuauExpr, Vec<Stmt>), value: (LuauExpr, Vec<Stmt>)) {
        let ((mut left, left_statements), (right, right_statements)) = (key, value);
        if !left_statements.is_empty() || !right_statements.is_empty() {
            self.disable_inline(cx);
            for statement in left_statements {
                cx.prereq(statement);
            }
            left = cx.push_to_var(left, "left");
        }
        for statement in right_statements {
            cx.prereq(statement);
        }
        self.assign(cx, left, right);
    }
}
