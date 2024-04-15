#[derive(Clone)]
pub struct PositionInfo {
    pub start_month_and_year: String,
    pub title: String,
    pub company: String,
    pub company_url: Option<String>,
    pub description: String,
}

impl PositionInfo {
    pub fn new(
        title: &'static str,
        company: &'static str,
        company_url: Option<&'static str>,
        start_month_and_year: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            title: title.into(),
            company: company.into(),
            company_url: company_url.map(|url| url.into()),
            start_month_and_year: start_month_and_year.into(),
            description: description.into(),
        }
    }
}
