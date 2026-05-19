use crate::{DisplayishResult, LevelLike, SealedTraitFunParam};
use core::fmt::{Debug, Display};

#[cfg(feature = "proc-macro2-diagnostics")]
use crate::{MacroDeepResult, MacroDiagnosticResult};

#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2::Span;

#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2_diagnostics::{Diagnostic as PmDiagnostic, SpanDiagnosticExt as _};

//------

pub trait DebugExt: Debug {
    fn dbg_error_at(&self, span: Span) -> PmDiagnostic;

    fn dbg_error_with_at<FD: Display, F: Fn() -> FD>(self, f: F, span: Span) -> PmDiagnostic;

    #[allow(private_interfaces)]
    fn _seal(&self, _: SealedTraitFunParam);
}
impl<T: Debug> DebugExt for T {
    fn dbg_error_at(&self, span: Span) -> PmDiagnostic {
        span.error(format!("{self:?}"))
    }
    fn dbg_error_with_at<FD: Display, F: Fn() -> FD>(self, f: F, span: Span) -> PmDiagnostic {
        let s = format!("{} {:?}", f(), self);
        span.error(s)
    }

    #[allow(private_interfaces)]
    fn _seal(&self, _: SealedTraitFunParam) {}
}
//------

pub trait AsDeepMacroResult<T, D: Display> {
    #[allow(private_interfaces)]
    fn as_error(self) -> MacroDeepResult<T, D>;
}

impl<T, D: Display> AsDeepMacroResult<T, D> for DisplayishResult<T, D> {
    #[allow(private_interfaces)]
    fn as_error(self) -> MacroDeepResult<T, D> {
        self.map_err(|err| err.and_extra(LevelLike))
    }
}
//------

pub trait ToStringExt: ToString {
    fn to_error_at(&self, span: Span) -> PmDiagnostic;
    fn to_error_with_at<FD: Display, F: Fn() -> FD>(&self, f: F, span: Span) -> PmDiagnostic;

    #[allow(private_interfaces)]
    fn _seal(&self, _: SealedTraitFunParam);
}
impl<T: ToString> ToStringExt for T {
    fn to_error_at(&self, span: Span) -> PmDiagnostic {
        span.error(self.to_string())
    }
    fn to_error_with_at<FD: Display, F: Fn() -> FD>(&self, f: F, span: Span) -> PmDiagnostic {
        let s = format!("{} {}", f(), self.to_string());
        span.error(s)
    }
    #[allow(private_interfaces)]
    fn _seal(&self, _: SealedTraitFunParam) {}
}
//------
pub trait ResultErrToDisplayExt<T> {
    fn map_error_to_at(self, span: Span) -> MacroDiagnosticResult<T>;
    fn map_error_to_with_at<FD: Display, F: Fn() -> FD>(
        self,
        f: F,
        span: Span,
    ) -> MacroDiagnosticResult<T>;

    #[allow(private_interfaces)]
    fn _seal(&self, _: SealedTraitFunParam);
}
impl<T, ERR: ToString> ResultErrToDisplayExt<T> for Result<T, ERR> {
    fn map_error_to_at(self, span: Span) -> MacroDiagnosticResult<T> {
        self.map_err(|e| span.error(e.to_string()))
    }
    #[cfg(feature = "proc-macro2-diagnostics")]
    fn map_error_to_with_at<FD: Display, F: Fn() -> FD>(
        self,
        f: F,
        span: Span,
    ) -> MacroDiagnosticResult<T> {
        self.map_err(|e| {
            let s = format!("{} {}", f(), e.to_string());
            span.error(s)
        })
    }

    #[allow(private_interfaces)]
    fn _seal(&self, _: SealedTraitFunParam) {}
}

//------
//------

pub trait MacroDeepResultExt<T> {
    // @TODO if implemented in proc_macro2_diagnostics, make it accept MultiSpan.
    /// Add the given [Span], and transform to [MacroResult].
    fn spanned(self, span: Span) -> MacroDiagnosticResult<T>;

    #[allow(private_interfaces)]
    fn _seal(&self, _: SealedTraitFunParam);
}
impl<T, D: Display> MacroDeepResultExt<T> for MacroDeepResult<T, D> {
    fn spanned(self, span: Span) -> MacroDiagnosticResult<T> {
        self.map_err(|deep_err| deep_err.spanned(span).into_diagnostic())
    }
    #[allow(private_interfaces)]
    fn _seal(&self, _: SealedTraitFunParam) {}
}
//------

//------
//------
//------
