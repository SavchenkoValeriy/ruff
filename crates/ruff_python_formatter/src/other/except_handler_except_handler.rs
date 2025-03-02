use ruff_formatter::FormatRuleWithOptions;
use ruff_formatter::{write, Buffer, FormatResult};
use ruff_python_ast::ExceptHandlerExceptHandler;

use crate::comments::SourceComment;
use crate::expression::maybe_parenthesize_expression;
use crate::expression::parentheses::Parenthesize;
use crate::prelude::*;
use crate::statement::clause::{clause_header, ClauseHeader};
use crate::{FormatNodeRule, PyFormatter};

#[derive(Copy, Clone, Default)]
pub enum ExceptHandlerKind {
    #[default]
    Regular,
    Starred,
}

#[derive(Default)]
pub struct FormatExceptHandlerExceptHandler {
    except_handler_kind: ExceptHandlerKind,
}

impl FormatRuleWithOptions<ExceptHandlerExceptHandler, PyFormatContext<'_>>
    for FormatExceptHandlerExceptHandler
{
    type Options = ExceptHandlerKind;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.except_handler_kind = options;
        self
    }
}

impl FormatNodeRule<ExceptHandlerExceptHandler> for FormatExceptHandlerExceptHandler {
    fn fmt_fields(
        &self,
        item: &ExceptHandlerExceptHandler,
        f: &mut PyFormatter,
    ) -> FormatResult<()> {
        let ExceptHandlerExceptHandler {
            range: _,
            type_,
            name,
            body,
        } = item;

        let comments_info = f.context().comments().clone();
        let dangling_comments = comments_info.dangling(item);

        write!(
            f,
            [
                clause_header(
                    ClauseHeader::ExceptHandler(item),
                    dangling_comments,
                    &format_with(|f| {
                        write!(
                            f,
                            [
                                text("except"),
                                match self.except_handler_kind {
                                    ExceptHandlerKind::Regular => None,
                                    ExceptHandlerKind::Starred => Some(text("*")),
                                }
                            ]
                        )?;

                        if let Some(type_) = type_ {
                            write!(
                                f,
                                [
                                    space(),
                                    maybe_parenthesize_expression(
                                        type_,
                                        item,
                                        Parenthesize::IfBreaks
                                    )
                                ]
                            )?;
                            if let Some(name) = name {
                                write!(f, [space(), text("as"), space(), name.format()])?;
                            }
                        }

                        Ok(())
                    }),
                ),
                block_indent(&body.format())
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _dangling_comments: &[SourceComment],
        _f: &mut PyFormatter,
    ) -> FormatResult<()> {
        // dangling comments are formatted as part of fmt_fields
        Ok(())
    }
}
