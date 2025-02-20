// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! MIR Span related functions

use crate::codegen_cprover_gotoc::GotocCtx;
use cbmc::goto_program::Location;
use rustc_smir::rustc_internal;
use rustc_span::Span;
use stable_mir::ty::Span as SpanStable;

impl<'tcx> GotocCtx<'tcx> {
    pub fn codegen_span(&self, sp: &Span) -> Location {
        self.codegen_span_stable(rustc_internal::stable(sp))
    }

    pub fn codegen_span_stable(&self, sp: SpanStable) -> Location {
        let loc = sp.get_lines();
        Location::new(
            sp.get_filename().to_string(),
            self.current_fn.as_ref().map(|x| x.readable_name().to_string()),
            loc.start_line,
            Some(loc.start_col),
            loc.end_line,
            Some(loc.end_col),
        )
    }

    pub fn codegen_span_option_stable(&self, sp: Option<SpanStable>) -> Location {
        sp.map_or(Location::none(), |span| self.codegen_span_stable(span))
    }

    pub fn codegen_caller_span_stable(&self, sp: SpanStable) -> Location {
        self.codegen_caller_span(&rustc_internal::internal(sp))
    }

    /// Get the location of the caller. This will attempt to reach the macro caller.
    /// This function uses rustc_span methods designed to returns span for the macro which
    /// originally caused the expansion to happen.
    /// Note: The API stops backtracing at include! boundary.
    pub fn codegen_caller_span(&self, span: &Span) -> Location {
        let topmost = span.ctxt().outer_expn().expansion_cause().unwrap_or(*span);
        self.codegen_span(&topmost)
    }
}
