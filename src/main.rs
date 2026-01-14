use std::{process::Command, time::Duration};
use thirtyfour::error::WebDriverResult;
use thirtyfour::extensions::query::ElementQueryable;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use tokio::time::sleep;

const PORT: u16 = 4444;

struct ChildGuard(std::process::Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // run geckodriver
    let geckodriver = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "geckodriver".into());

    let _gecko = ChildGuard(
        Command::new(geckodriver)
            .arg("--port")
            .arg(PORT.to_string())
            .spawn()
            .expect("failed to start geckodriver"),
    );

    // setup webdriver
    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;

    // retry connect a few times to avoid startup race
    let server = format!("http://localhost:{PORT}");
    let mut driver = None;
    for _ in 0..10 {
        match WebDriver::new(&server, caps.clone()).await {
            Ok(d) => {
                driver = Some(d);
                break;
            }
            Err(_) => sleep(Duration::from_millis(150)).await,
        }
    }
    let driver = driver.expect("could not connect to geckodriver");

    let result: WebDriverResult<()> = async {
        driver
            .goto("https://guest-1.ufvisitor.ufl.edu/index.php")
            .await?;

        // more resilient than a raw find() on slow pages
        let button = driver.query(By::Id("submit")).first().await?;
        button.click().await?;
        Ok(())
    }
    .await;

    // try to quit even if something failed
    let _ = driver.quit().await;
    result
}
