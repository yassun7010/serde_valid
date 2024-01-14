use unic_langid_impl::LanguageIdentifier;

type FluentBundle =
    serde_valid::export::fluent::FluentBundle<serde_valid::export::fluent::FluentResource>;

pub trait FluentState {
    fn get_fluent_bundle(&self) -> Option<&FluentBundle>;

    fn get_fluent_bundle_on_lang(&self, lang: LanguageIdentifier) -> Option<&FluentBundle>;
}

impl<T> FluentState for T {
    fn get_fluent_bundle(&self) -> Option<&FluentBundle> {
        None
    }

    fn get_fluent_bundle_on_lang(&self, _lang: LanguageIdentifier) -> Option<&FluentBundle> {
        None
    }
}
