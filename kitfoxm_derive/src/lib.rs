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
                        use super::#name;
                        use impls::impls;
                        use kitfoxm::actions::{Resource};

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
                    } // impl resource
                } // paste
            }// macro match case
        } // macro-rules actions

        // FINDME: New Actions
        // Add every new action that a Resource can support to this macro.
        //
        // SupportedActions is required here but is also implemented manually by the derive
        // macro so that a default implementation can be provided.
        actions!(
            SupportedActions,
            Identifier,
            Identify,
            Health,
            Erase
        );
    }; // quote

    gen.into()
}

#[proc_macro_derive(Resource)]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_resource(&ast)
}
