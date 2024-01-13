#[cfg(feature = "fluent")]
mod tests {
    use fluent_0::{FluentBundle, FluentResource};
    use serde::Deserialize;
    use serde_json::json;
    use serde_valid::{fluent::Localize, Validate};
    use unic_langid::LanguageIdentifier;

    fn get_bundle() -> FluentBundle<FluentResource> {
        let ftl_string = ["hello-world = Hello, world!", "intro = Welcome, { $name }."]
            .join("\n")
            .to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res).unwrap();

        bundle
    }

    #[test]
    fn fluent_error() {
        #[derive(Debug, Deserialize, Validate)]
        struct Test {
            #[validate(minimum = 5, fluent("hello-world"))]
            a: u32,
            #[validate(maximum = 10, fluent("intro", name = "taro"))]
            b: u32,
        }

        let test = Test { a: 1, b: 11 };
        let a = test.validate().unwrap_err().localize(&get_bundle());
        assert_eq!(
            a.to_string(),
            json!({
                "errors": [],
                "properties": {
                    "a": {
                        "errors": [
                            "Hello, world!"
                        ]
                    },
                    "b": {
                        "errors": [
                            "Welcome, \u{2068}taro\u{2069}."
                        ]
                    }
                }
            })
            .to_string()
        );
    }
}
