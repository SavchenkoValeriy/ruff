use ruff_formatter::{format_args, write};
use ruff_python_ast::node::AstNode;
use ruff_python_ast::{Ranged, Stmt, StmtWhile};

use crate::comments::SourceComment;
use crate::expression::maybe_parenthesize_expression;
use crate::expression::parentheses::Parenthesize;
use crate::prelude::*;
use crate::statement::clause::{clause_header, ClauseHeader, ElseClause};
use crate::FormatNodeRule;

#[derive(Default)]
pub struct FormatStmtWhile;

impl FormatNodeRule<StmtWhile> for FormatStmtWhile {
    fn fmt_fields(&self, item: &StmtWhile, f: &mut PyFormatter) -> FormatResult<()> {
        let StmtWhile {
            range: _,
            test,
            body,
            orelse,
        } = item;

        let comments = f.context().comments().clone();
        let dangling_comments = comments.dangling(item.as_any_node_ref());

        let body_start = body.first().map_or(test.end(), Stmt::start);
        let or_else_comments_start =
            dangling_comments.partition_point(|comment| comment.slice().end() < body_start);

        let (trailing_condition_comments, or_else_comments) =
            dangling_comments.split_at(or_else_comments_start);

        write!(
            f,
            [
                clause_header(
                    ClauseHeader::While(item),
                    trailing_condition_comments,
                    &format_args![
                        text("while"),
                        space(),
                        maybe_parenthesize_expression(test, item, Parenthesize::IfBreaks),
                    ]
                ),
                block_indent(&body.format())
            ]
        )?;

        if !orelse.is_empty() {
            // Split between leading comments before the `else` keyword and end of line comments at the end of
            // the `else:` line.
            let trailing_start =
                or_else_comments.partition_point(|comment| comment.line_position().is_own_line());
            let (leading, trailing) = or_else_comments.split_at(trailing_start);

            write!(
                f,
                [
                    clause_header(
                        ClauseHeader::OrElse(ElseClause::While(item)),
                        trailing,
                        &text("else")
                    )
                    .with_leading_comments(leading, body.last()),
                    block_indent(&orelse.format())
                ]
            )?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(
        &self,
        _dangling_comments: &[SourceComment],
        _f: &mut PyFormatter,
    ) -> FormatResult<()> {
        // Handled in `fmt_fields`
        Ok(())
    }
}
