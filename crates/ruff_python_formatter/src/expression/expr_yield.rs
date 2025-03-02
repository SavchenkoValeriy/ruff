use crate::context::PyFormatContext;
use crate::expression::maybe_parenthesize_expression;
use crate::expression::parentheses::{NeedsParentheses, OptionalParentheses, Parenthesize};
use crate::prelude::*;
use crate::{FormatNodeRule, PyFormatter};
use ruff_formatter::write;
use ruff_python_ast::node::AnyNodeRef;
use ruff_python_ast::{Expr, ExprYield, ExprYieldFrom, Ranged};
use ruff_text_size::TextRange;

pub(super) enum AnyExpressionYield<'a> {
    Yield(&'a ExprYield),
    YieldFrom(&'a ExprYieldFrom),
}

impl<'a> AnyExpressionYield<'a> {
    const fn is_yield_from(&self) -> bool {
        matches!(self, AnyExpressionYield::YieldFrom(_))
    }

    fn value(&self) -> Option<&Expr> {
        match self {
            AnyExpressionYield::Yield(yld) => yld.value.as_deref(),
            AnyExpressionYield::YieldFrom(yld) => Some(&yld.value),
        }
    }
}

impl Ranged for AnyExpressionYield<'_> {
    fn range(&self) -> TextRange {
        match self {
            AnyExpressionYield::Yield(yld) => yld.range(),
            AnyExpressionYield::YieldFrom(yld) => yld.range(),
        }
    }
}

impl NeedsParentheses for AnyExpressionYield<'_> {
    fn needs_parentheses(
        &self,
        parent: AnyNodeRef,
        _context: &PyFormatContext,
    ) -> OptionalParentheses {
        // According to https://docs.python.org/3/reference/grammar.html There are two situations
        // where we do not want to always parenthesize a yield expression:
        //  1. Right hand side of an assignment, e.g. `x = yield y`
        //  2. Yield statement, e.g. `def foo(): yield y`
        // We catch situation 1 below. Situation 2 does not need to be handled here as
        // FormatStmtExpr, does not add parenthesis
        if parent.is_stmt_assign() || parent.is_stmt_ann_assign() || parent.is_stmt_aug_assign() {
            OptionalParentheses::Multiline
        } else {
            OptionalParentheses::Always
        }
    }
}

impl<'a> From<&'a ExprYield> for AnyExpressionYield<'a> {
    fn from(value: &'a ExprYield) -> Self {
        AnyExpressionYield::Yield(value)
    }
}

impl<'a> From<&'a ExprYieldFrom> for AnyExpressionYield<'a> {
    fn from(value: &'a ExprYieldFrom) -> Self {
        AnyExpressionYield::YieldFrom(value)
    }
}

impl<'a> From<&AnyExpressionYield<'a>> for AnyNodeRef<'a> {
    fn from(value: &AnyExpressionYield<'a>) -> Self {
        match value {
            AnyExpressionYield::Yield(yld) => AnyNodeRef::ExprYield(yld),
            AnyExpressionYield::YieldFrom(yld) => AnyNodeRef::ExprYieldFrom(yld),
        }
    }
}

impl Format<PyFormatContext<'_>> for AnyExpressionYield<'_> {
    fn fmt(&self, f: &mut PyFormatter) -> FormatResult<()> {
        let keyword = if self.is_yield_from() {
            "yield from"
        } else {
            "yield"
        };

        if let Some(val) = self.value() {
            write!(
                f,
                [
                    text(keyword),
                    space(),
                    maybe_parenthesize_expression(val, self, Parenthesize::IfRequired)
                ]
            )?;
        } else {
            // ExprYieldFrom always has Some(value) so we should never get a bare `yield from`
            write!(f, [&text(keyword)])?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct FormatExprYield;

impl FormatNodeRule<ExprYield> for FormatExprYield {
    fn fmt_fields(&self, item: &ExprYield, f: &mut PyFormatter) -> FormatResult<()> {
        AnyExpressionYield::from(item).fmt(f)
    }
}

impl NeedsParentheses for ExprYield {
    fn needs_parentheses(
        &self,
        parent: AnyNodeRef,
        context: &PyFormatContext,
    ) -> OptionalParentheses {
        AnyExpressionYield::from(self).needs_parentheses(parent, context)
    }
}
