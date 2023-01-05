use super::error::Error;
use paste::paste;
use serde::{Deserialize, Serialize};

macro_rules! actions {
    ( $($x:ident $(Result {$($y:ident: $z:ty), *})? $(Args {$($a:ident: $b:ty), *})? $(CompleteArgs {$($c:ident: $d:ty), *})? $(ProgressResult {$($e:ident: $f:ty), *})? $(ProgressArgs {$($g:ident: $h:ty), *})? $(CancelResult {$($i:ident: $j:ty), *})? $(CancelArgs {$($k:ident: $l:ty), *})? $(PauseResult {$($m:ident: $n:ty), *})? $(PauseArgs {$($o:ident: $p:ty), *})? $(ResumeResult {$($q:ident: $r:ty), *})? $(ResumeArgs {$($s: ident: $t:ty), *})?), *) => {
        paste! {
            pub trait Resource {
                fn action(&self, payload: &ActionPayload) -> ActionPayload;
                $(
                    fn [<send_ $x:snake>](&self, args: &[<$x Args>]) -> [<$x Result>];
                    fn [<send_ $x:snake _complete>](&self, args: &[<$x CompleteArgs>]) -> [<$x CompleteResult>];
                    fn [<send_ $x:snake _progress>](&self, args: &[<$x ProgressArgs>]) -> [<$x ProgressResult>];
                    fn [<send_ $x:snake _cancel>](&self, args: &[<$x CancelArgs>]) -> [<$x CancelResult>];
                    fn [<send_ $x:snake _pause>](&self, args: &[<$x PauseArgs>]) -> [<$x PauseResult>];
                    fn [<send_ $x:snake _resume>](&self, args: &[<$x ResumeArgs>]) -> [<$x ResumeResult>];
                )*
                fn send_supported_actions(&self, args: &SupportedActionsArgs) -> SupportedActionsResult;
                fn send_supported_actions_complete(&self, args: &SupportedActionsCompleteArgs) -> SupportedActionsCompleteResult;
                fn send_supported_actions_progress(&self, args: &SupportedActionsProgressArgs) -> SupportedActionsProgressResult;
                fn send_supported_actions_cancel(&self, args: &SupportedActionsCancelArgs) -> SupportedActionsCancelResult;
                fn send_supported_actions_pause(&self, args: &SupportedActionsPauseArgs) -> SupportedActionsPauseResult;
                fn send_supported_actions_resume(&self, args: &SupportedActionsResumeArgs) -> SupportedActionsResumeResult;
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct RequestPayload {
                pub payload: Vec<IdentifierPayload>,
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct IdentifierPayload {
                pub identifier: ResourceIdentifier,
                pub payload: Vec<ActionPayload>,
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            #[serde(tag = "type")]
            pub enum ActionPayload {
                #[default]
                Unsupported,
                SupportedActionsArgs(SupportedActionsArgs),
                SupportedActionsCompleteArgs(SupportedActionsCompleteArgs),
                SupportedActionsProgressArgs(SupportedActionsProgressArgs),
                SupportedActionsCancelArgs(SupportedActionsCancelArgs),
                SupportedActionsPauseArgs(SupportedActionsPauseArgs),
                SupportedActionsResumeArgs(SupportedActionsResumeArgs),
                SupportedActionsResult(SupportedActionsResult),
                SupportedActionsCompleteResult(SupportedActionsCompleteResult),
                SupportedActionsProgressResult(SupportedActionsProgressResult),
                SupportedActionsCancelResult(SupportedActionsCancelResult),
                SupportedActionsPauseResult(SupportedActionsPauseResult),
                SupportedActionsResumeResult(SupportedActionsResumeResult),
                $(
                    [<$x Args>]([<$x Args>]),
                    [<$x CompleteArgs>]([<$x CompleteArgs>]),
                    [<$x ProgressArgs>]([<$x ProgressArgs>]),
                    [<$x CancelArgs>]([<$x CancelArgs>]),
                    [<$x PauseArgs>]([<$x PauseArgs>]),
                    [<$x ResumeArgs>]([<$x ResumeArgs>]),
                    [<$x Result>]([<$x Result>]),
                    [<$x CompleteResult>]([<$x CompleteResult>]),
                    [<$x ProgressResult>]([<$x ProgressResult>]),
                    [<$x CancelResult>]([<$x CancelResult>]),
                    [<$x PauseResult>]([<$x PauseResult>]),
                    [<$x ResumeResult>]([<$x ResumeResult>]),
                )*
            }

            $(
                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x Result>] {
                    pub args: Option<[<$x Args>]>,
                    pub operation_identifier: Option<Result<OperationIdentifier, Error>>,
                    $($(
                        pub $y: Option<Result<$z, Error>>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x Args>] {
                    pub asynchronous: Option<bool>,
                    $($(
                        pub $a: Option<$b>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x CompleteResult>] {
                    pub args: Option<[<$x Args>]>,
                    pub result: Option<Result<[<$x Result>], Error>>
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x CompleteArgs>] {
                    $($(
                        pub $c: Option<Result<$d>>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x ProgressResult>] {
                    pub args: Option<[<$x Args>]>,
                    pub progress: Option<Result<u32, Error>>,
                    $($(
                        pub $e: Option<Result<$f, Error>>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x ProgressArgs>] {
                    $($(
                        pub $g: Option<$h>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x CancelResult>] {
                    pub args: Option<[<$x Args>]>,
                    pub cancelled: Option<Result<bool, Error>>,
                    $($(
                        pub $i: Option<Result<$j, Error>>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x CancelArgs>] {
                    $($(
                        pub $k: Option<$l>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x PauseResult>] {
                    pub args: Option<[<$x Args>]>,
                    pub paused: Option<Result<bool, Error>>,
                    $($(
                        pub $m: Option<Result<$n, Error>>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x PauseArgs>] {
                    $($(
                        pub $o: Option<$p>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x ResumeResult>] {
                    pub args: Option<[<$x Args>]>,
                    pub resumed: Option<Result<bool, Error>>,
                    $($(
                        pub $q: Option<Result<$r, Error>>,
                    )*)?
                }

                #[derive(Serialize, Deserialize, Default, Debug)]
                pub struct [<$x ResumeArgs>] {
                    $($(
                        pub $s: Option<Result<$t, Error>>,
                    )*)?
                }

                pub trait $x {
                    fn [<$x:snake>](&self, args: &[<$x Args>]) -> [<$x Result>];
                    fn [<$x:snake _complete>](&self, args: &[<$x CompleteArgs>]) -> [<$x CompleteResult>] {
                        _ = args;
                        [<$x CompleteResult>] { result: Some(Err(Error::not_implemented())), ..Default::default() }
                    }
                    fn [<$x:snake _progress>](&self, args: &[<$x ProgressArgs>]) -> [<$x ProgressResult>]
                    {
                        _ = args;
                        [<$x ProgressResult>] { progress: Some(Err(Error::not_implemented())), ..Default::default() }
                    }
                    fn [<$x:snake _cancel>](&self, args: &[<$x CancelArgs>]) -> [<$x CancelResult>] {
                        _ = args;
                        [<$x CancelResult>] { cancelled: Some(Err(Error::not_implemented())), ..Default::default() }
                    }
                    fn [<$x:snake _pause>](&self, args: &[<$x PauseArgs>]) -> [<$x PauseResult>] {
                        _ = args;
                        [<$x PauseResult>] { paused: Some(Err(Error::not_implemented())), ..Default::default() }
                    }
                    fn [<$x:snake _resume>](&self, args: &[<$x ResumeArgs>]) -> [<$x ResumeResult>] {
                        _ = args;
                        [<$x ResumeResult>] { resumed: Some(Err(Error::not_implemented())), ..Default::default() }
                    }
                }
            )*

            // SupportedActions is a special action which is implemented by every resource.
            // It needs to be defined separately because the SupportedActionsResult structure
            // needs to know about all other actions in order to be defined, and we do not
            // want to repeat ourselves below so instead we will repeat ourselves here.
            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsResult {
                pub operation_identifier: Option<Result<OperationIdentifier, Error>>,
                pub supported_actions: Option<Result<bool, Error>>,
                $(
                    pub [<$x:snake>]: Option<Result<bool, Error>>,
                )*
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsArgs {
                pub asynchronous: Option<bool>,
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsCompleteResult {
                pub result: Option<Result<SupportedActionsResult, Error>>,
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsCompleteArgs {}

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsProgressResult {
                pub progress: Option<Result<u32, Error>>,
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsProgressArgs {}

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsCancelResult {
                pub cancelled: Option<Result<bool, Error>>
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsCancelArgs {}

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsPauseResult {
                pub paused: Option<Result<bool, Error>>,
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsPauseArgs {}

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsResumeResult {
                pub resumed: Option<Result<bool, Error>>
            }

            #[derive(Serialize, Deserialize, Default, Debug)]
            pub struct SupportedActionsResumeArgs {}

            pub trait SupportedActions {
                // This method is implemented for the specific type when deriving Resource.
                fn supported_actions(&self, args: &SupportedActionsArgs) -> SupportedActionsResult;

                fn supported_actions_complete(&self, args: &SupportedActionsCompleteArgs) -> SupportedActionsCompleteResult {
                    _ = args;
                    SupportedActionsCompleteResult { result: Some(Err(Error::not_implemented())), ..Default::default() }
                }

                fn supported_actions_progress(&self, args: &SupportedActionsProgressArgs) -> SupportedActionsProgressResult {
                    _ = args;
                    SupportedActionsProgressResult { progress: Some(Err(Error::not_implemented())), ..Default::default() }
                }

                fn supported_actions_cancel(&self, args: &SupportedActionsCancelArgs) -> SupportedActionsCancelResult {
                    _ = args;
                    SupportedActionsCancelResult { cancelled: Some(Err(Error::not_implemented())), ..Default::default() }
                }

                fn supported_actions_pause(&self, args: &SupportedActionsPauseArgs) -> SupportedActionsPauseResult {
                    _ = args;
                    SupportedActionsPauseResult { paused: Some(Err(Error::not_implemented())), ..Default::default() }
                }

                fn supported_actions_resume(&self, args: &SupportedActionsResumeArgs) -> SupportedActionsResumeResult {
                    _ = args;
                    SupportedActionsResumeResult { resumed: Some(Err(Error::not_implemented())), ..Default::default() }
                }
            }
        }
    };
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct OperationIdentifier(String);

#[derive(
    Serialize, Deserialize, Default, Debug, Clone, Hash, std::cmp::Eq, std::cmp::PartialEq,
)]
pub struct ResourceIdentifier(pub String);

// FINDME: New Actions
// Define a new action by adding to this macro. This is tightly coupled to the
// implementation in the Resource derive macro. Using the macro lets you
// specify the important bits while taking care of all the boilerplate and naming
// requirements.

actions!(
    Identifier
    Result {
        identifier: ResourceIdentifier
    },

    Identify
    Result {
        name: String
    },

    Erase
);
