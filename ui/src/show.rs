use k8s::openapi::jiff;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Padding;
use tabled::settings::Remove;
use tabled::settings::Settings;
use tabled::settings::Style;
use tabled::settings::location::ByColumnName;
use tabled::settings::style::On;

use super::*;

pub use params::ShowParams;

mod impls;
mod params;

type TableSettings = Settings<Settings<Settings, Style<(), (), (), (), (), On, 0, 0>>, Padding>;

pub trait Show {
    fn header(&self, output: OutputFormat) -> Vec<String>;
    fn data(&self, params: ShowParams, output: OutputFormat) -> Vec<String>;
    fn json(&self, params: ShowParams) -> String;
    fn yaml(&self, params: ShowParams) -> String;
    fn name(&self) -> String;

    fn normal(&self, params: ShowParams, output: OutputFormat) -> Table {
        tabled::builder::Builder::from_iter([self.header(output), self.data(params, output)])
            .build()
    }

    fn wide(&self, params: ShowParams, output: OutputFormat) -> Table {
        tabled::builder::Builder::from_iter([self.header(output), self.data(params, output)])
            .build()
    }

    fn go_template(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn go_template_file(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn template(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn template_file(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn json_path(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn json_path_as_json(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn json_path_file(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn custom_columns(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn custom_columns_file(&self) -> String {
        todo!("This method is not supported yet")
    }

    fn output(&self, namespace: bool, params: ShowParams, output: OutputFormat) -> String {
        match output {
            OutputFormat::Normal => {
                let mut table = self.normal(params, output);
                table.with(self.table_settings());
                if !namespace {
                    table.with(Remove::column(ByColumnName::new("NAMESPACE")));
                }
                table.to_string()
            }
            OutputFormat::Json => self.json(params),
            OutputFormat::Yaml => self.yaml(params),
            OutputFormat::Name => self.name().to_string(),
            OutputFormat::GoTemplate => self.go_template(),
            OutputFormat::GoTemplateFile => self.go_template_file(),
            OutputFormat::Template => self.template(),
            OutputFormat::TemplateFile => self.template_file(),
            OutputFormat::JsonPath => self.json_path(),
            OutputFormat::JsonPathAsJson => self.json_path_as_json(),
            OutputFormat::JsonPathFile => self.json_path_file(),
            OutputFormat::CustomColumns => self.custom_columns(),
            OutputFormat::CustomColumnsFile => self.custom_columns_file(),
            OutputFormat::Wide => {
                let mut table = self.wide(params, output);
                table.with(self.table_settings());
                if !namespace {
                    table.with(Remove::column(ByColumnName::new("NAMESPACE")));
                }
                table.to_string()
            }
        }
    }

    fn table_settings(&self) -> TableSettings {
        Settings::empty()
            .with(Style::blank())
            // .with(Padding::zero())
            .with(Padding::new(0, 2, 0, 0))
    }
}

impl<T> Show for Vec<T>
where
    T: Show,
{
    fn header(&self, output: OutputFormat) -> Vec<String> {
        todo!("Not supported on Vec<T> for {output:?}")
    }

    fn data(&self, _params: ShowParams, output: OutputFormat) -> Vec<String> {
        todo!("Not supported on Vec<T> for {output:?}")
    }

    fn normal(&self, params: ShowParams, output: OutputFormat) -> tabled::Table {
        let header = self.first().map(|t| t.header(output));
        let data = self.iter().map(|t| t.data(params, output));
        let builder = header
            .into_iter()
            .chain(data)
            .collect::<tabled::builder::Builder>();
        builder.build()
    }

    fn wide(&self, params: ShowParams, output: OutputFormat) -> Table {
        let header = self.first().map(|t| t.header(output));
        let data = self.iter().map(|t| t.data(params, output));
        let builder = header
            .into_iter()
            .chain(data)
            .collect::<tabled::builder::Builder>();
        builder.build()
    }

    fn yaml(&self, params: ShowParams) -> String {
        self.iter()
            .map(|item| item.yaml(params))
            .collect::<Vec<_>>()
            .join("")
    }

    fn json(&self, params: ShowParams) -> String {
        self.iter()
            .map(|item| item.json(params))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn name(&self) -> String {
        todo!()
    }
}

fn name<K>(object: &K, params: ShowParams) -> String
where
    K: kube::Resource,
    K::DynamicType: Default,
{
    let name = object.name_any();
    if params.show_kind {
        let kind = K::kind(&default()).to_lowercase();
        format!("{kind}/{name}")
    } else {
        name
    }
}

fn age(time: metav1::Time) -> String {
    let delta = jiff::Timestamp::now() - time.0;
    human_time(delta)
}

/// Mimics k8s humantime printer. See
/// https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/util/duration/duration.go
fn human_time(delta: jiff::Span) -> String {
    // Get total seconds directly from the original span
    let options = jiff::SpanTotal::from(jiff::Unit::Second).days_are_24_hours();
    let seconds = match delta.total(options) {
        Ok(seconds) => seconds as i64,
        Err(_) => return "<invalid>".to_string(),
    };

    // Precalculate all time units
    let minutes = seconds / 60;
    let hours = seconds / 3600;
    let days = hours / 24;
    let years = days / 365;

    // Allow deviation no more than 2 seconds(excluded) to tolerate machine time
    // inconsistence, it can be considered as almost now.
    if seconds < -1 {
        "<invalid>".to_string()
    } else if seconds < 0 {
        "0s".to_string()
    } else if seconds < 60 * 2 {
        format!("{seconds}s")
    } else if minutes < 10 {
        let seconds = seconds % 60;
        if seconds == 0 {
            format!("{minutes}m")
        } else {
            format!("{minutes}m{seconds}s")
        }
    } else if minutes < 60 * 3 {
        format!("{minutes}m")
    } else if hours < 8 {
        let minutes = minutes % 60;
        if minutes == 0 {
            format!("{hours}h")
        } else {
            format!("{hours}h{minutes}m")
        }
    } else if hours < 48 {
        format!("{hours}h")
    } else if hours < 24 * 8 {
        let hours = hours % 24;
        if hours == 0 {
            format!("{days}d")
        } else {
            format!("{days}d{hours}h")
        }
    } else if hours < 24 * 365 * 2 {
        format!("{days}d")
    } else if hours < 24 * 365 * 8 {
        let days = days % 365;
        if days == 0 {
            format!("{years}y")
        } else {
            format!("{years}y{days}d")
        }
    } else {
        format!("{years}y")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::ToSpan;

    trait SpanExt {
        fn less_milli(self) -> jiff::Span;
    }

    impl SpanExt for jiff::Span {
        fn less_milli(self) -> jiff::Span {
            let options = jiff::SpanArithmetic::from(1.millisecond()).days_are_24_hours();
            self.checked_sub(options).unwrap()
        }
    }

    #[test]
    fn human_time_basic() {
        assert_eq!(human_time(1.seconds()), "1s");
        assert_eq!(human_time(70.seconds()), "70s");
        assert_eq!(human_time(190.seconds()), "3m10s");
        assert_eq!(human_time(70.minutes()), "70m");
        assert_eq!(human_time(47.hours()), "47h");
        assert_eq!(human_time(49.hours()), "2d1h");
        assert_eq!(human_time((8 * 24 + 2).hours()), "8d");
        assert_eq!(human_time((367 * 24).hours()), "367d");
        assert_eq!(human_time((365 * 2 * 24 + 25).hours()), "2y1d");
        assert_eq!(human_time((365 * 8 * 24 + 2).hours()), "8y");
    }

    #[test]
    fn human_time_boundary() -> Result<(), jiff::Error> {
        // Negative times
        assert_eq!(human_time(-2.seconds()), "<invalid>");
        assert_eq!(human_time(-1.seconds()), "0s");
        assert_eq!(human_time(0.seconds()), "0s");
        assert_eq!(human_time(1.seconds().less_milli()), "0s");

        // Seconds boundary (< 2 minutes)
        assert_eq!(human_time(2.minutes().less_milli()), "119s");
        assert_eq!(human_time(2.minutes()), "2m");
        assert_eq!(human_time(2.minutes().seconds(1)), "2m1s");

        // Minutes boundary (< 10 minutes)
        assert_eq!(human_time(10.minutes().less_milli()), "9m59s");
        assert_eq!(human_time(10.minutes()), "10m");
        assert_eq!(human_time(10.minutes().seconds(1)), "10m");

        // Minutes boundary (< 3 hours)
        assert_eq!(human_time(3.hours().less_milli()), "179m");
        assert_eq!(human_time(3.hours()), "3h");
        assert_eq!(human_time(3.hours().minutes(1)), "3h1m");

        // Hours boundary (< 8 hours)
        assert_eq!(human_time(8.hours().less_milli()), "7h59m");
        assert_eq!(human_time(8.hours()), "8h");
        assert_eq!(human_time(8.hours().minutes(59)), "8h");

        // Hours boundary (< 48 hours)
        assert_eq!(human_time(2.days().less_milli()), "47h");
        assert_eq!(human_time(2.days()), "2d");
        assert_eq!(human_time(2.days().hours(1)), "2d1h");

        // Days boundary (< 8 days)
        assert_eq!(human_time(8.days().less_milli()), "7d23h");
        assert_eq!(human_time(8.days()), "8d");
        assert_eq!(human_time(8.days().hours(23)), "8d");

        // Years boundary (< 2 years)
        assert_eq!(human_time((2 * 365).days().less_milli()), "729d");
        assert_eq!(human_time((2 * 365).days()), "2y");
        assert_eq!(human_time((2 * 365).days().hours(23)), "2y");
        assert_eq!(human_time((2 * 365).days().hours(23).minutes(59)), "2y");
        assert_eq!(human_time((2 * 365 + 1).days().less_milli()), "2y");
        assert_eq!(human_time((2 * 365 + 1).days()), "2y1d");

        // Years boundary (< 8 years)
        assert_eq!(human_time((3 * 365).days()), "3y");
        assert_eq!(human_time((4 * 365).days()), "4y");
        assert_eq!(human_time((5 * 365).days()), "5y");
        assert_eq!(human_time((6 * 365).days()), "6y");
        assert_eq!(human_time((7 * 365).days()), "7y");
        assert_eq!(human_time((8 * 365).days().less_milli()), "7y364d");
        assert_eq!(human_time((8 * 365).days()), "8y");
        assert_eq!(human_time((8 * 365 + 364).days()), "8y");
        assert_eq!(human_time((9 * 365).days()), "9y");

        Ok(())
    }
}
