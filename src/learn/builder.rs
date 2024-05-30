use chrono::{Date, DateTime, TimeZone, Utc};

#[derive(Debug, Clone)]
pub struct Details {
    given_name: String,
    preferred_name: Option<String>,
    middle_name: Option<String>,
    family_name: String,
    mobile_phone_e164: Option<String>,
    dob: Date<Utc>,
    last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct DetailsBuilder(Details);

impl DetailsBuilder {
    fn new(given_name: &str, family_name: &str, dob: Date<Utc>) -> Self {
        DetailsBuilder(Details {
            given_name: given_name.to_owned(),
            preferred_name: None,
            middle_name: None,
            family_name: family_name.to_owned(),
            mobile_phone_e164: None,
            dob,
            last_seen: None,
        })
    }

    fn preferred_name(&mut self, preferred_name: &str) -> &mut Self {
        self.0.preferred_name = Some(preferred_name.to_owned());
        self
    }

    fn middle_name(&mut self, middle_name: &str) -> &mut Self {
        self.0.middle_name = Some(middle_name.to_owned());
        self
    }

    fn just_seen(&mut self) -> &mut Self {
        self.0.last_seen = Some(Utc::now());
        self
    }

    fn build(&mut self) -> Details {
        self.0.clone()
    }
}

pub fn test() {
    let _ = DetailsBuilder::new("Robert", "Builder", chrono::Utc.ymd(1998, 11, 28))
        .middle_name("the")
        .preferred_name("Bob")
        .just_seen()
        .build();
}

pub fn test1() {
    let mut builder = DetailsBuilder::new("Robert", "Builder", chrono::Utc.ymd(1998, 11, 28));
    builder.middle_name("the").just_seen();
    builder.preferred_name("Bob");
    let bob = builder.build();
}

pub fn test2() {
    let mut smithy = DetailsBuilder::new("Agent", "Smith", chrono::Utc.ymd(1999, 6, 11));
    let clones = vec![smithy.build(), smithy.build(), smithy.build()];
}
