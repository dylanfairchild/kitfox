use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

fn impl_resource(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mod_name = format_ident!("{}_mod_impl", name);

    let gen = quote! {
        use paste::paste;

        macro_rules! actions {
            ( $( $x:ident ),* ) => {
                paste!{
                    #[allow(non_snake_case)]
                    mod #mod_name {
                        use uuid;
                        use super::#name;
                        use impls::impls;
                        use kitfoxm::actions::{Resource, ResourceIdentifier};

                        struct Wrap<T>(T);

                        $(
                            use kitfoxm::actions::{
                                $x,
                                [<$x Args>],
                                [<$x Result>],
                                [<$x CompleteArgs>],
                                [<$x CompleteResult>],
                                [<$x ProgressArgs>],
                                [<$x ProgressResult>],
                                [<$x CancelArgs>],
                                [<$x CancelResult>],
                                [<$x PauseArgs>],
                                [<$x PauseResult>],
                                [<$x ResumeArgs>],
                                [<$x ResumeResult>]
                            };

                            trait [<$x Supported>] {
                                fn [<send_ $x:snake>](&self, args: &[<$x Args>]) -> [<$x Result>];
                                fn [<send_ $x:snake _complete>](&self, args: &[<$x CompleteArgs>]) -> [<$x CompleteResult>];
                                fn [<send_ $x:snake _progress>](&self, args: &[<$x ProgressArgs>]) -> [<$x ProgressResult>];
                                fn [<send_ $x:snake _cancel>](&self, args: &[<$x CancelArgs>]) -> [<$x CancelResult>];
                                fn [<send_ $x:snake _pause>](&self, args: &[<$x PauseArgs>]) -> [<$x PauseResult>];
                                fn [<send_ $x:snake _resume>](&self, args: &[<$x ResumeArgs>]) -> [<$x ResumeResult>];
                            }
                            impl<T: $x> [<$x Supported>] for &Wrap<&T> {
                                fn [<send_ $x:snake>](&self, args: &[<$x Args>]) -> [<$x Result>] {
                                    T::[<$x:snake>](&self.0, args)
                                }
                                fn [<send_ $x:snake _complete>](&self, args: &[<$x CompleteArgs>]) -> [<$x CompleteResult>] {
                                    T::[<$x:snake _complete>](&self.0, args)
                                }
                                fn [<send_ $x:snake _progress>](&self, args: &[<$x ProgressArgs>]) -> [<$x ProgressResult>] {
                                    T::[<$x:snake _progress>](&self.0, args)
                                }
                                fn [<send_ $x:snake _cancel>](&self, args: &[<$x CancelArgs>]) -> [<$x CancelResult>] {
                                    T::[<$x:snake _cancel>](&self.0, args)
                                }
                                fn [<send_ $x:snake _pause>](&self, args: &[<$x PauseArgs>]) -> [<$x PauseResult>] {
                                    T::[<$x:snake _pause>](&self.0, args)
                                }
                                fn [<send_ $x:snake _resume>](&self, args: &[<$x ResumeArgs>]) -> [<$x ResumeResult>] {
                                    T::[<$x:snake _resume>](&self.0, args)
                                }
                            }

                            trait [<$x Unsupported>] {
                                fn [<send_ $x:snake>](&self, args: &[<$x Args>]) -> [<$x Result>];
                                fn [<send_ $x:snake _complete>](&self, args: &[<$x CompleteArgs>]) -> [<$x CompleteResult>];
                                fn [<send_ $x:snake _progress>](&self, args: &[<$x ProgressArgs>]) -> [<$x ProgressResult>];
                                fn [<send_ $x:snake _cancel>](&self, args: &[<$x CancelArgs>]) -> [<$x CancelResult>];
                                fn [<send_ $x:snake _pause>](&self, args: &[<$x PauseArgs>]) -> [<$x PauseResult>];
                                fn [<send_ $x:snake _resume>](&self, args: &[<$x ResumeArgs>]) -> [<$x ResumeResult>];
                            }
                            impl<T> [<$x Unsupported>] for Wrap<&T> {
                                fn [<send_ $x:snake>](&self, _args: &[<$x Args>]) -> [<$x Result>] {
                                    Default::default()
                                }
                                fn [<send_ $x:snake _complete>](&self, _args: &[<$x CompleteArgs>]) -> [<$x CompleteResult>] {
                                    Default::default()
                                }
                                fn [<send_ $x:snake _progress>](&self, _args: &[<$x ProgressArgs>]) -> [<$x ProgressResult>] {
                                    Default::default()
                                }
                                fn [<send_ $x:snake _cancel>](&self, _args: &[<$x CancelArgs>]) -> [<$x CancelResult>] {
                                    Default::default()
                                }
                                fn [<send_ $x:snake _pause>](&self, _args: &[<$x PauseArgs>]) -> [<$x PauseResult>] {
                                    Default::default()
                                }
                                fn [<send_ $x:snake _resume>](&self, _args: &[<$x ResumeArgs>]) -> [<$x ResumeResult>] {
                                    Default::default()
                                }
                            }

                            pub(super) fn [<send_ $x:snake _impl>](s: &#name, args: &[<$x Args>]) -> [<$x Result>] {
                                (&&(Wrap(s))).[<send_ $x:snake>](args)
                            }
                            pub(super) fn [<send_ $x:snake _complete_impl>](s: &#name, args: &[<$x CompleteArgs>]) -> [<$x CompleteResult>] {
                                (&&(Wrap(s))).[<send_ $x:snake _complete>](args)
                            }
                            pub(super) fn [<send_ $x:snake _progress_impl>](s: &#name, args: &[<$x ProgressArgs>]) -> [<$x ProgressResult>] {
                                (&&(Wrap(s))).[<send_ $x:snake _progress>](args)
                            }
                            pub(super) fn [<send_ $x:snake _cancel_impl>](s: &#name, args: &[<$x CancelArgs>]) -> [<$x CancelResult>] {
                                (&&(Wrap(s))).[<send_ $x:snake _cancel>](args)
                            }
                            pub(super) fn [<send_ $x:snake _pause_impl>](s: &#name, args: &[<$x PauseArgs>]) -> [<$x PauseResult>] {
                                (&&(Wrap(s))).[<send_ $x:snake _pause>](args)
                            }
                            pub(super) fn [<send_ $x:snake _resume_impl>](s: &#name, args: &[<$x ResumeArgs>]) -> [<$x ResumeResult>] {
                                (&&(Wrap(s))).[<send_ $x:snake _resume>](args)
                            }
                        )*

                        use kitfoxm::actions::{
                            Identifier,
                            IdentifierArgs,
                            IdentifierResult,
                            IdentifierCompleteArgs,
                            IdentifierCompleteResult,
                            IdentifierProgressArgs,
                            IdentifierProgressResult,
                            IdentifierCancelArgs,
                            IdentifierCancelResult,
                            IdentifierPauseArgs,
                            IdentifierPauseResult,
                            IdentifierResumeArgs,
                            IdentifierResumeResult
                        };

                        trait IdentifierSupported {
                            fn send_identifier(&self, args: &IdentifierArgs) -> IdentifierResult;
                            fn send_identifier_complete(&self, args: &IdentifierCompleteArgs) -> IdentifierCompleteResult;
                            fn send_identifier_progress(&self, args: &IdentifierProgressArgs) -> IdentifierProgressResult;
                            fn send_identifier_cancel(&self, args: &IdentifierCancelArgs) -> IdentifierCancelResult;
                            fn send_identifier_pause(&self, args: &IdentifierPauseArgs) -> IdentifierPauseResult;
                            fn send_identifier_resume(&self, args: &IdentifierResumeArgs) -> IdentifierResumeResult;
                        }
                        impl <T: Identifier> IdentifierSupported for &Wrap<&T> {
                            fn send_identifier(&self, args: &IdentifierArgs) -> IdentifierResult {
                                T::identifier(&self.0, args)
                            }
                            fn send_identifier_complete(&self, args: &IdentifierCompleteArgs) -> IdentifierCompleteResult {
                                T::identifier_complete(&self.0, args)
                            }
                            fn send_identifier_progress(&self, args: &IdentifierProgressArgs) -> IdentifierProgressResult {
                                T::identifier_progress(&self.0, args)
                            }
                            fn send_identifier_cancel(&self, args: &IdentifierCancelArgs) -> IdentifierCancelResult {
                                T::identifier_cancel(&self.0, args)
                            }
                            fn send_identifier_pause(&self, args: &IdentifierPauseArgs) -> IdentifierPauseResult {
                                T::identifier_pause(&self.0, args)
                            }
                            fn send_identifier_resume(&self, args: &IdentifierResumeArgs) -> IdentifierResumeResult {
                                T::identifier_resume(&self.0, args)
                            }
                        }

                        trait IdentifierUnsupported {
                            fn send_identifier(&self, args: &IdentifierArgs) -> IdentifierResult;
                            fn send_identifier_complete(&self, args: &IdentifierCompleteArgs) -> IdentifierCompleteResult;
                            fn send_identifier_progress(&self, args: &IdentifierProgressArgs) -> IdentifierProgressResult;
                            fn send_identifier_cancel(&self, args: &IdentifierCancelArgs) -> IdentifierCancelResult;
                            fn send_identifier_pause(&self, args: &IdentifierPauseArgs) -> IdentifierPauseResult;
                            fn send_identifier_resume(&self, args: &IdentifierResumeArgs) -> IdentifierResumeResult;
                        }
                        impl <T> IdentifierUnsupported for Wrap<&T> {
                            fn send_identifier(&self, _args: &IdentifierArgs) -> IdentifierResult {
                                IdentifierResult { identifier: Some(Ok(ResourceIdentifier(String::from(format!("{}", uuid::Uuid::new_v4()))))), ..Default::default() }
                            }
                            fn send_identifier_complete(&self, _args: &IdentifierCompleteArgs) -> IdentifierCompleteResult {
                                Default::default()
                            }
                            fn send_identifier_progress(&self, _args: &IdentifierProgressArgs) -> IdentifierProgressResult {
                                Default::default()
                            }
                            fn send_identifier_cancel(&self, _args: &IdentifierCancelArgs) -> IdentifierCancelResult {
                                Default::default()
                            }
                            fn send_identifier_pause(&self, _args: &IdentifierPauseArgs) -> IdentifierPauseResult {
                                Default::default()
                            }
                            fn send_identifier_resume(&self, _args: &IdentifierResumeArgs) -> IdentifierResumeResult {
                                Default::default()
                            }
                        }

                        pub(super) fn send_identifier_impl(s: &#name, args: &IdentifierArgs) -> IdentifierResult {
                            (&&(Wrap(s))).send_identifier(args)
                        }
                        pub(super) fn send_identifier_complete_impl(s: &#name, args: &IdentifierCompleteArgs) -> IdentifierCompleteResult {
                            (&&(Wrap(s))).send_identifier_complete(args)
                        }
                        pub(super) fn send_identifier_progress_impl(s: &#name, args: &IdentifierProgressArgs) -> IdentifierProgressResult {
                            (&&(Wrap(s))).send_identifier_progress(args)
                        }
                        pub(super) fn send_identifier_cancel_impl(s: &#name, args: &IdentifierCancelArgs) -> IdentifierCancelResult {
                            (&&(Wrap(s))).send_identifier_cancel(args)
                        }
                        pub(super) fn send_identifier_pause_impl(s: &#name, args: &IdentifierPauseArgs) -> IdentifierPauseResult {
                            (&&(Wrap(s))).send_identifier_pause(args)
                        }
                        pub(super) fn send_identifier_resume_impl(s: &#name, args: &IdentifierResumeArgs) -> IdentifierResumeResult {
                            (&&(Wrap(s))).send_identifier_resume(args)
                        }

                        impl SupportedActions for #name {
                            fn supported_actions(&self, _args: &SupportedActionsArgs) -> SupportedActionsResult {
                                SupportedActionsResult {
                                    $(
                                        [<$x:snake>]: Some(Ok(impls!(#name: $x))),
                                    )*
                                    ..Default::default()
                                }
                            }
                        }

                        use kitfoxm::actions::ActionPayload;

                        pub(super) fn action_impl(s: &#name, args: &ActionPayload) -> ActionPayload {
                            match args {
                                $(
                                    ActionPayload::[<$x Args>](args) => ActionPayload::[<$x Result>](s.[<send_ $x:snake>](args)),
                                    ActionPayload::[<$x CompleteArgs>](args) => ActionPayload::[<$x CompleteResult>](s.[<send_ $x:snake _complete>](args)),
                                    ActionPayload::[<$x ProgressArgs>](args) => ActionPayload::[<$x ProgressResult>](s.[<send_ $x:snake _progress>](args)),
                                    ActionPayload::[<$x CancelArgs>](args) => ActionPayload::[<$x CancelResult>](s.[<send_ $x:snake _cancel>](args)),
                                    ActionPayload::[<$x PauseArgs>](args) => ActionPayload::[<$x PauseResult>](s.[<send_ $x:snake _pause>](args)),
                                    ActionPayload::[<$x ResumeArgs>](args) => ActionPayload::[<$x ResumeResult>](s.[<send_ $x:snake _resume>](args)),
                                )*
                                ActionPayload::IdentifierArgs(args) => ActionPayload::IdentifierResult(s.send_identifier(args)),
                                ActionPayload::IdentifierCompleteArgs(args) => ActionPayload::IdentifierCompleteResult(s.send_identifier_complete(args)),
                                ActionPayload::IdentifierProgressArgs(args) => ActionPayload::IdentifierProgressResult(s.send_identifier_progress(args)),
                                ActionPayload::IdentifierCancelArgs(args) => ActionPayload::IdentifierCancelResult(s.send_identifier_cancel(args)),
                                ActionPayload::IdentifierPauseArgs(args) => ActionPayload::IdentifierPauseResult(s.send_identifier_pause(args)),
                                ActionPayload::IdentifierResumeArgs(args) => ActionPayload::IdentifierResumeResult(s.send_identifier_resume(args)),
                                _ => Default::default()
                            }
                        }
                    } // mod

                    impl Resource for #name {
                        fn action(&self, payload: &kitfoxm::actions::ActionPayload) -> kitfoxm::actions::ActionPayload {
                            #mod_name::action_impl(&self, payload)
                        }
                        $(
                            fn [<send_ $x:snake>](&self, args: &kitfoxm::actions::[<$x Args>]) -> kitfoxm::actions::[<$x Result>] {
                                #mod_name::[<send_ $x:snake _impl>](&self, args)
                            }
                            fn [<send_ $x:snake _complete>](&self, args: &kitfoxm::actions::[<$x CompleteArgs>]) -> kitfoxm::actions::[<$x CompleteResult>] {
                                #mod_name::[<send_ $x:snake _complete_impl>](&self, args)
                            }
                            fn [<send_ $x:snake _progress>](&self, args: &kitfoxm::actions::[<$x ProgressArgs>]) -> kitfoxm::actions::[<$x ProgressResult>] {
                                #mod_name::[<send_ $x:snake _progress_impl>](&self, args)
                            }
                            fn [<send_ $x:snake _cancel>](&self, args: &kitfoxm::actions::[<$x CancelArgs>]) -> kitfoxm::actions::[<$x CancelResult>] {
                                #mod_name::[<send_ $x:snake _cancel_impl>](&self, args)
                            }
                            fn [<send_ $x:snake _pause>](&self, args: &kitfoxm::actions::[<$x PauseArgs>]) -> kitfoxm::actions::[<$x PauseResult>] {
                                #mod_name::[<send_ $x:snake _pause_impl>](&self, args)
                            }
                            fn [<send_ $x:snake _resume>](&self, args: &kitfoxm::actions::[<$x ResumeArgs>]) -> kitfoxm::actions::[<$x ResumeResult>] {
                                #mod_name::[<send_ $x:snake _resume_impl>](&self, args)
                            }
                        )*
                        fn send_identifier(&self, args: &kitfoxm::actions::IdentifierArgs) -> kitfoxm::actions::IdentifierResult {
                            #mod_name::send_identifier_impl(&self, args)
                        }
                        fn send_identifier_complete(&self, args: &kitfoxm::actions::IdentifierCompleteArgs) -> kitfoxm::actions::IdentifierCompleteResult {
                            #mod_name::send_identifier_complete_impl(&self, args)
                        }
                        fn send_identifier_progress(&self, args: &kitfoxm::actions::IdentifierProgressArgs) -> kitfoxm::actions::IdentifierProgressResult {
                            #mod_name::send_identifier_progress_impl(&self, args)
                        }
                        fn send_identifier_cancel(&self, args: &kitfoxm::actions::IdentifierCancelArgs) -> kitfoxm::actions::IdentifierCancelResult {
                            #mod_name::send_identifier_cancel_impl(&self, args)
                        }
                        fn send_identifier_pause(&self, args: &kitfoxm::actions::IdentifierPauseArgs) -> kitfoxm::actions::IdentifierPauseResult {
                            #mod_name::send_identifier_pause_impl(&self, args)
                        }
                        fn send_identifier_resume(&self, args: &kitfoxm::actions::IdentifierResumeArgs) -> kitfoxm::actions::IdentifierResumeResult {
                            #mod_name::send_identifier_resume_impl(&self, args)
                        }
                    } // impl resource
                } // paste
            }// macro match case
        } // macro-rules actions

        // FINDME: New Actions
        // Add every new action that a Resource can support to this macro.
        //
        // Identifier action is not required, that is implemented manually by the derive
        // macro so that a default implementation can be provided.
        //
        // SupportedActions is required here but is also implemented manually by the derive
        // macro so that a default implementation can be provided.
        actions!(SupportedActions, Identify, Erase);
    }; // quote

    gen.into()
}

#[proc_macro_derive(Resource)]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_resource(&ast)
}
