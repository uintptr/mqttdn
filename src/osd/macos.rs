use anyhow::Result;
use log::info;
use mac_notification_sys::{get_bundle_identifier_or_default, send_notification, set_application};

use crate::osd::OsdTrait;

pub struct MacOsd {}

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

impl OsdTrait for MacOsd {
    fn new() -> Result<MacOsd> {
        let bundle = get_bundle_identifier_or_default("hello");

        info!("bundle: {bundle}");

        set_application(&bundle)?;

        Ok(MacOsd {})
    }

    fn display<S>(&self, text: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        info!("notification: {}", text.as_ref());
        send_notification(PKG_NAME, None, text.as_ref(), None)?;
        Ok(())
    }
}
