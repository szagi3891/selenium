use thirtyfour::prelude::*;
use tokio;
// use tokio::prelude::*;
use std::time::Duration;

/*

https://dev.to/stevepryde/using-selenium-with-rust-aca


https://mkyong.com/java/how-to-install-java-on-mac-osx/
brew tap adoptopenjdk/openjdk
brew search jdk
brew cask install adoptopenjdk11

java -jar selenium-server-standalone-3.141.59.jar


CHROME DRIVER - uruchamianie z internetu
https://support.apple.com/pl-pl/guide/mac-help/mh40616/mac


https://medium.com/@HoussemDellai/run-selenium-ui-tests-in-docker-containers-f41ae2796b8d

docker run -d -p 4444:4444 -v /dev/shm:/dev/shm selenium/standalone-chrome:3.141.59-yttrium
*/


#[tokio::main]
async fn main() {
    println!("aaa1");

    color_eyre::install().unwrap();

    println!("aaa2");

    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation").unwrap();
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps).await.unwrap();

    println!("aaa3");

    driver.get("https://wikipedia.org/").await.unwrap();
    //https://www.wikipedia.org/

    println!("aaa4 - 1");

    tokio::time::delay_for(Duration::from_secs(5)).await;

    println!("aaa4 - 2");

    //let elem_search = driver.find_element(By::Name("h1")).await.unwrap();

    let elem_search = driver.find_element(By::Id("searchInput")).await.unwrap();
    elem_search.send_keys("selenium").await.unwrap();

    tokio::time::delay_for(Duration::from_secs(5)).await;
    
    println!("aaa5");

    let ggg = elem_search.inner_html().await.unwrap();

    println!("----> {}", ggg);
    
    println!("aaa6");
    // Ok(())
}

