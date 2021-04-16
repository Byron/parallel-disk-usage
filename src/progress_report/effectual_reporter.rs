use super::{Event, Progress, ProgressReport, Size};
use crate::error_report::ErrorReport;
use pipe_trait::Pipe;
use std::sync::{Arc, RwLock};

/// Store progress information and call report function on said information.
#[derive(Debug)]
pub struct EffectualReporter<Data, ReportProgress, ReportError>
where
    Data: Size,
    ReportProgress: Fn(&Progress<Data>) + Sync,
    ReportError: Fn(ErrorReport) + Sync,
{
    /// Progress information.
    pub progress: Arc<RwLock<Progress<Data>>>,
    /// Report progress information.
    pub report_progress: ReportProgress,
    /// Report encountered error.
    pub report_error: ReportError,
}

impl<Data, ReportProgress, ReportError> EffectualReporter<Data, ReportProgress, ReportError>
where
    Data: Size,
    ReportProgress: Fn(&Progress<Data>) + Sync,
    ReportError: Fn(ErrorReport) + Sync,
{
    /// Create a new [`EffectualReporter`] from a report function.
    pub fn new(report_progress: ReportProgress, report_error: ReportError) -> Self
    where
        Progress<Data>: Default,
    {
        let progress = Progress::default().pipe(RwLock::new).pipe(Arc::new);
        EffectualReporter {
            progress,
            report_progress,
            report_error,
        }
    }
}

impl<Data, ReportProgress, ReportError> ProgressReport<Data>
    for EffectualReporter<Data, ReportProgress, ReportError>
where
    Data: Size,
    ReportProgress: Fn(&Progress<Data>) + Sync,
    ReportError: Fn(ErrorReport) + Sync,
{
    fn report(&self, event: Event<Data>) {
        use Event::*;
        let EffectualReporter {
            progress,
            report_progress,
            report_error,
        } = self;
        macro_rules! handle_field {
            ($field:ident $operator:tt $addend:expr) => {{
                {
                    let expect_message = concat!("lock progress to mutate", stringify!($field));
                    let mut progress = progress.write().expect(expect_message);
                    progress.$field $operator $addend;
                }
                {
                    let progress = progress.read().expect("lock progress to report");
                    report_progress(&progress);
                }
            }};

            ($field:ident) => {
                handle_field!($field += 1);
            };
        }
        match event {
            BeginScanning => handle_field!(known_items),
            FinishScanning => handle_field!(scanned_items),
            ReceiveData(data) => handle_field!(scanned_total += data),
            EncounterError(error_report) => {
                report_error(error_report);
                handle_field!(errors)
            }
        }
    }
}
