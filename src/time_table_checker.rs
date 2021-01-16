use crate::StoqedOptions;
use notify_rust::Notification;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

const REFRESH_INTERVAL: Duration = Duration::from_secs(30);
const TIME_TABLE_URL: &str = "https://apps.es.vt.edu/ssb/HZSKVTSC.P_ProcRequest";

pub struct TimeTableChecker {
    session_id: String,
    term: u32,
    crns: Vec<u32>,
    client: Client,
}

fn show_notif(crn: &u32) {
    Notification::new()
        .summary("stoqed")
        .body(format!("Class CRN {} is available!", crn).as_str())
        .timeout(5)
        .show()
        .expect("unable to show notification");
}

impl TimeTableChecker {
    pub fn new(opts: StoqedOptions) -> TimeTableChecker {
        TimeTableChecker {
            session_id: opts.session_id,
            term: opts.term_year,
            crns: opts.crns,
            client: reqwest::blocking::Client::new(),
        }
    }

    fn check_classes(self) -> Result<(), Box<dyn Error>> {
        loop {
            self.crns
                .iter()
                .filter(|crn| self.has_seat_available(crn).unwrap())
                .for_each(|crn| show_notif(crn));

            sleep(REFRESH_INTERVAL);
        }
    }
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        self.check_classes()?;

        Ok(())
    }

    fn has_seat_available(&self, crn: &&u32) -> Result<bool, Box<dyn Error>> {
        log::info!("checking CRN {}", crn);

        let mut headers = HeaderMap::new();
        let sessid_cookie_str = format!("SESSID={};", self.session_id);

        // Set SESSID cookie, set content-type
        headers.insert(COOKIE, HeaderValue::from_str(sessid_cookie_str.as_str())?);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/x-www-form-urlencoded")?,
        );

        let test = self
            .client
            .post(TIME_TABLE_URL)
            .headers(headers)
            .body(format!(
                "CAMPUS=0&TERMYEAR={}&CORE_CODE=AR%25&subj_code=%25&SCHDTYPE=%25&\
                           CRSE_NUMBER=&crn={}&open_only=&disp_comments_in=Y&sess_code=%25&\
                           BTN_PRESSED=FIND+class+sections&inst_name=",
                self.term, crn
            ))
            .send()?;

        let content = test.text()?;

        if !content.contains(">Seats") {
            return Err("Invalid SESSID".into());
        }

        Ok(!content.contains(">Full"))
    }
}
