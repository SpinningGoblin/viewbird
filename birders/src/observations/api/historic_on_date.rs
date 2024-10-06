use time::{Date, Month};

use crate::{errors::BirderError, observations::Observation, Birders};

use super::HistoricOnDateParams;

pub struct HistoricOnDateHandler<'birder> {
    birder: &'birder Birders,
    region_code: String,
    date: Date,
    params: Option<HistoricOnDateParams>,
}

impl<'birder> HistoricOnDateHandler<'birder> {
    pub fn new(
        birder: &'birder Birders,
        region_code: &str,
        date: &Date,
        params: Option<HistoricOnDateParams>,
    ) -> Self {
        Self {
            birder,
            params,
            region_code: region_code.to_string(),
            date: *date,
        }
    }
}

fn month_to_num(month: &Month) -> u8 {
    match month {
        Month::January => 1,
        Month::February => 2,
        Month::March => 3,
        Month::April => 4,
        Month::May => 5,
        Month::June => 6,
        Month::July => 7,
        Month::August => 8,
        Month::September => 9,
        Month::October => 10,
        Month::November => 11,
        Month::December => 12,
    }
}

impl<'birder> HistoricOnDateHandler<'birder> {
    pub async fn get(&self) -> Result<Vec<Observation>, BirderError> {
        let url = format!(
            "/data/obs/{}/historic/{}/{}/{}",
            self.region_code,
            self.date.year(),
            month_to_num(&self.date.month()),
            self.date.day()
        );

        let full_url = match &self.params {
            Some(it) => format!("{}?{}", url, it.to_url().join("&")),
            None => url,
        };

        self.birder.get(&full_url).await
    }
}
