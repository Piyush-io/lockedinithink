// **** 1. Sequential reqwests using ureq; ****
// use std::sync::Arc;
// use std::time;
// use tokio::sync::Semaphore;
// // fn main() -> Result<(), ureq::Error> {
// //     let start_time = time::Instant::now();
// //     let links: [&str; 20] = [
// //         "https://example.com",
// //         "https://httpbin.org/get",
// //         "https://jsonplaceholder.typicode.com/posts",
// //         "https://jsonplaceholder.typicode.com/posts/1",
// //         "https://jsonplaceholder.typicode.com/users",
// //         "https://api.github.com",
// //         "https://api.github.com/repos/rust-lang/rust",
// //         "https://raw.githubusercontent.com/rust-lang/rust/master/README.md",
// //         "https://dog.ceo/api/breeds/list/all",
// //         "https://catfact.ninja/fact",
// //         "https://www.testing.com/",
// //         "https://www.timeanddate.com/worldclock/timezone/utc",
// //         "https://api.agify.io/?name=michael",
// //         "https://api.genderize.io/?name=alex",
// //         "https://api.nationalize.io/?name=arjun",
// //         "https://pokeapi.co/api/v2/pokemon/ditto",
// //         "https://api.spacexdata.com/v4/launches/latest",
// //         "https://api.open-meteo.com/v1/forecast?latitude=28.6&longitude=77.2&current_weather=true",
// //         "https://www.rust-lang.org",
// //         "https://bruh.xyz/", // Test request timeouts (2-second delay)
// //     ];
// //     for (i, &link) in links.iter().enumerate() {
// //         println!("Link {}: {}", i + 1, link);
// //         println!("--------------------------");
// //         let body = ureq::get(link).call()?.body_mut().read_to_string()?;
// //         println!("{}", body);
// //         println!("--------------------------");
// //     }
// //     let elapsed_time = start_time.elapsed();
// //     println!("Elapsed time: {:?}", elapsed_time);
// //     Ok(())
// // }
// //

//  **** 2. 3. 4. Parallel reqwests using a single client with timeout and response code handling
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let start_time = time::Instant::now();
//     let client: reqwest::Client = reqwest::Client::new();
//     let semaphore = Arc::new(Semaphore::new(50));
//     let mut handles = Vec::new();
//     for (i, &link) in LINKS.iter().enumerate() {
//         let sem = Arc::clone(&semaphore);
//         let client_handle = client.clone();
//         println!("Link {}: {}", i + 1, link);
//         let handle = tokio::spawn(async move {
//             let _permit = sem.acquire().await.expect("Waiting for semaphore");
//             client_handle
//                 .get(link)
//                 .timeout(time::Duration::new(5, 0))
//                 .send()
//                 .await?
//                 .error_for_status()?
//                 .text()
//                 .await?;
//             Ok::<_, reqwest::Error>((i, link))
//         });
//         handles.push(handle);
//         println!("--------------------------");
//     }

//     for handle in handles {
//         match handle.await {
//             Ok(Ok((index, url))) => {
//                 println!("Link {}: {}", index + 1, url);
//                 println!("--------------------------");
//             }
//             Ok(Err(err)) => {
//                 eprintln!("Error fetching link: {}", err);
//             }
//             Err(err) => {
//                 panic!("Task Panicked: {}", err);
//             }
//         };
//     }

//     let elapsed_time = start_time.elapsed();
//     println!("Elapsed time: {:?}", elapsed_time);

//     // TODO for Phase 1 Task 5:
//     // 1. Create a HashSet<String> to track seen URLs (prevents duplicate crawling)
//     // 2. Create a VecDeque<String> as the URL frontier (queue for BFS traversal)
//     // 3. Initialize frontier with seed URLs from LINKS array
//     // 4. Instead of processing all URLs at once, implement a loop that:
//     //    - Pops URLs from the frontier queue
//     //    - Checks if they've been seen before
//     //    - If not seen, adds them to seen set and fetches the page
//     //    - After fetching, extracts all <a href> links from HTML response
//     //    - Adds unseen extracted links to the frontier queue
//     // 5. Implement proper BFS traversal logic using the frontier queue
//     // 6. Replace the current batch processing with gradual frontier consumption
//     // 7. Add HTML parsing capability to extract links (may need scraper or similar crate)

//     Ok(())
// }

// // 5. Frontier
// use scraper::{Html, Selector};
// use std::collections::{HashSet, VecDeque};
// use url::Url;
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let seed_urls = vec![
//         "https://raw.githubusercontent.com/rust-lang/rust/master/README.md",
//         "https://dog.ceo/api/breeds/list/all",
//         "https://catfact.ninja/fact",
//         "https://www.testing.com/",
//         "https://www.timeanddate.com/worldclock/timezone/utc",
//         "https://api.agify.io/?name=michael",
//         "https://api.genderize.io/?name=alex",
//         "https://api.nationalize.io/?name=arjun",
//         "https://pokeapi.co/api/v2/pokemon/ditto",
//         "https://api.spacexdata.com/v4/launches/latest",
//         "https://api.open-meteo.com/v1/forecast?latitude=28.6&longitude=77.2&current_weather=true",
//         "https://www.rust-lang.org",
//         "https://bruh.xyz/", // Test request timeouts (2-second delay)
//         "https://example.com",
//         "https://httpbin.org/get",
//         "https://jsonplaceholder.typicode.com/posts",
//         "https://jsonplaceholder.typicode.com/posts/1",
//         "https://jsonplaceholder.typicode.com/users",
//     ];
//     let mut seen: HashSet<String> = HashSet::new();
//     let mut frontier: VecDeque<String> = seed_urls.into_iter().map(String::from).collect();

//     while let Some(link) = frontier.pop_front() {
//         if seen.contains(&link) {
//             continue;
//         }
//         println!("Processing : {}", &link);
//         seen.insert(link.clone());
//         let body = match reqwest::get(&link).await {
//             Ok(resp) if resp.status().is_success() => resp.text().await?,
//             _ => continue,
//         };
//         let selector = Selector::parse("a[href]").unwrap();
//         let document = Html::parse_document(&body);
//         for element in document.select(&selector) {
//             if let Some(href) = element.attr("href") {
//                 let base = match Url::parse(&link) {
//                     Ok(b) => b,
//                     Err(e) => {
//                         eprintln!("Invalid base URL {}: {}", link, e);
//                         continue;
//                     }
//                 };

//                 match base.join(href) {
//                     Ok(absolute) => {
//                         let abs_str = absolute.to_string();
//                         // Very useful filter - skip non-http(s) schemes
//                         if abs_str.starts_with("http://") || abs_str.starts_with("https://") {
//                             // Optional: avoid adding same URL again (though seen will catch it later)
//                             if !seen.contains(&abs_str) {
//                                 frontier.push_back(abs_str);
//                             }
//                         }
//                     }
//                     Err(e) => {}
//                 }
//             }
//         }
//     }

//     Ok(())
// }
//

// 6. Persistent Frontier
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use url::Url;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed_urls = vec![
        "https://raw.githubusercontent.com/rust-lang/rust/master/README.md",
        "https://dog.ceo/api/breeds/list/all",
        "https://catfact.ninja/fact",
        "https://www.testing.com/",
        "https://www.timeanddate.com/worldclock/timezone/utc",
        "https://api.agify.io/?name=michael",
        "https://api.genderize.io/?name=alex",
        "https://api.nationalize.io/?name=arjun",
        "https://pokeapi.co/api/v2/pokemon/ditto",
        "https://api.spacexdata.com/v4/launches/latest",
        "https://api.open-meteo.com/v1/forecast?latitude=28.6&longitude=77.2&current_weather=true",
        "https://www.rust-lang.org",
        "https://bruh.xyz/", // Test request timeouts (2-second delay)
        "https://example.com",
        "https://httpbin.org/get",
        "https://jsonplaceholder.typicode.com/posts",
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/users",
    ];
    let mut seen: HashSet<String> = HashSet::new();
    let mut frontier: VecDeque<String> = seed_urls.into_iter().map(String::from).collect();

    while let Some(link) = frontier.pop_front() {
        if seen.contains(&link) {
            continue;
        }
        println!("Processing : {}", &link);
        seen.insert(link.clone());
        let body = match reqwest::get(&link).await {
            Ok(resp) if resp.status().is_success() => resp.text().await?,
            _ => continue,
        };
        let selector = Selector::parse("a[href]").unwrap();
        let document = Html::parse_document(&body);
        for element in document.select(&selector) {
            if let Some(href) = element.attr("href") {
                let base = match Url::parse(&link) {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("Invalid base URL {}: {}", link, e);
                        continue;
                    }
                };

                match base.join(href) {
                    Ok(absolute) => {
                        let abs_str = absolute.to_string();
                        // Very useful filter - skip non-http(s) schemes
                        if abs_str.starts_with("http://") || abs_str.starts_with("https://") {
                            // Optional: avoid adding same URL again (though seen will catch it later)
                            if !seen.contains(&abs_str) {
                                frontier.push_back(abs_str);
                            }
                        }
                    }
                    Err(e) => {}
                }
            }
        }
    }

    Ok(())
}
