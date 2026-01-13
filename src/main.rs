use std::process::Command;
use thirtyfour::prelude::{By, DesiredCapabilities, WebDriver, WebDriverResult};

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

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    let driver = WebDriver::new(&format!("http://localhost:{PORT}"), caps).await?;

    driver
        .goto("https://guest-1.ufvisitor.ufl.edu/index.php")
        .await?;
    let button = driver.find(By::Id("submit")).await?;
    button.click().await?;

    driver.quit().await?;

    Ok(())
}
