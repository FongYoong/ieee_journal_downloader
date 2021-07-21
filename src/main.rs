mod utils;
mod pdf_helper;
use scraper::{ Html, Selector };
use serde_json::{ Value };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RecordStore {
    records: Vec<Value>,
}

#[derive(Debug)]
pub enum DownloadSource {
    SCIHUB,
    LIBGEN,
}

use std::{thread, time};
use std::fs::File;
use std::io::Write;
use futures_util::StreamExt;
use std::cmp::min;
use std::fs;
//use tempdir::TempDir;

use reqwest::Url;
use indicatif::{ProgressBar, ProgressStyle};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use dialoguer::{
    Input,
    Select,
    Confirm,
    theme::ColorfulTheme
};
use console::Term;
use console::Emoji;
use console::style as TermStyle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stdout();
    term.set_title("IEEE Journal Downloader");

    loop {
        term.clear_screen()?;
        let mut journal_url = get_url();
        let ieee_punumber;
        let ieee_isnumber;
        loop {
            match parse_url(&journal_url) {
                Ok((a, b)) => {
                    ieee_punumber = a;
                    ieee_isnumber = b;
                    break;
                }
                Err(_) => {
                    term.clear_screen()?;
                    println!("{}", TermStyle("Invalid Link!").bold().red());
                    utils::print_divider(&term);
                    journal_url = get_url();
                }
            };
        }
        term.clear_screen()?;
        let download_source = get_download_source();

        term.clear_screen()?;
        println!("{}", TermStyle("Fetching metadata...").bold().yellow());

        // Get Records
        let client = reqwest::Client::new();
        let record_store = get_record_store(&client, &journal_url, &ieee_punumber, &ieee_isnumber).await.unwrap();
        let sample_record = &record_store.records[0];
        term.clear_screen()?;
        println!("Url: {}", TermStyle(journal_url).bright());
        println!("Title: {}", TermStyle(&sample_record["publicationTitle"]).bright());
        println!("Volume: {}", TermStyle(&sample_record["volume"]).bright());
        println!("Issue: {}", TermStyle(&sample_record["issue"]).bright());
        println!("Type: {}", TermStyle(&sample_record["contentType"]).bright());

        utils::print_divider(&term);

        // Download PDFs
        //let temp_directory = TempDir::new("temp")?;
        let publication_name = &sample_record["publicationTitle"];
        let issue_name = format!("Volume_{}_Issue_{}", sample_record["volume"], sample_record["issue"]);
        let output_directory_name = format_json_string(&format!("pdf_output/{}/{}", publication_name, issue_name));
        let output_file_name = format_json_string(&format!("{}.pdf", issue_name));
        let output_separate_path_string = format!("{}/separate/", output_directory_name);
        let output_separate_path = std::path::Path::new(output_separate_path_string.as_str());
        match fs::create_dir_all(&output_separate_path_string) {
            Err(_) => {
                println!("Failed to create directory!")
            }
            _ => {
    
            }
        };
        download_documents(&term, &client, &download_source, &record_store, output_separate_path).await.unwrap();

        // Merge PDFs
        println!("{}", TermStyle("Merging Documents...").bold().yellow());
        //let mut documents = pdf_helper::get_documents(std::path::Path::new("pdf_source"));
        let mut documents = pdf_helper::get_documents(output_separate_path);
        
        // pdf_helper::merge_documents(&mut documents, &output_directory_name, &output_file_name)
        // pdf_helper::merge_documents(&mut documents, "pdf_output", "test.pdf")
        // https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=9340528&punumber=8475037
        if pdf_helper::merge_documents(&mut documents, &output_directory_name, &output_file_name) {
            println!("\n{}", TermStyle("Success!").bold().green());
            println!("\n{}", TermStyle(format!("File saved to: {}/{}",
                format!("{}/{}", std::env::current_dir().unwrap().into_os_string().into_string().unwrap()
                , output_directory_name), output_file_name)).bold());
        }
        else {
            println!("\n{}", TermStyle("Failed to create PDF file!\nThis is most likely because the documents cannot be obtained at this moment.").bold().red());
        }
        
        utils::print_divider(&term);

        if !Confirm::new().with_prompt("Do you want to download another journal?").interact()? {
            println!("{}", TermStyle("Thank you! :)").bold());
            thread::sleep(time::Duration::from_millis(1000));
            break;
        }
    }
    Ok(())
}

pub fn get_url() -> String {
    let url;

    loop {
        let mut clipboard_context: ClipboardContext = ClipboardProvider::new().unwrap();
        let clipboard_content= match clipboard_context.get_contents(){
            Ok(value) => {
                value
            },
            Err(_) => {
                "(No valid clipboard value found.)".to_string()
            }
        };
        let clipboard_format = format!("Clipboard: {}", clipboard_content);
        let link_options = vec!["Type Link Manually", &clipboard_format];
        println!("{}", TermStyle("What's the link?").bold().yellow());
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&link_options)
            .default(0)
            .interact_on_opt(&Term::stderr()).unwrap();
        match selection {
            Some(index) => {
                url = match index {
                    0 => {
                        let input = Input::new()
                            .with_prompt("Please type link")
                            .with_initial_text("")
                            .interact_text();
                        let input_value = match input {
                            Ok(v) => {
                                v
                            },
                            Err(_) => {
                                continue;
                            }
                        };
                        input_value
                        
                    },
                    1 => {
                        clipboard_content
                    }
                    _ => {
                        panic!("Shouldn't reach here.")
                    }
                };
                break;
            },
            None => println!("You did not select anything")
        }
    }
    return url;
}

pub fn parse_url(url: &str) -> Result<(String, String), ()> {
    let url_object = match Url::parse(&url) {
        Ok(o) => {
            o
        },
        Err(_) => {
            return Err(());
        }
    };
    match url_object.query() {
        Some(query_string) => {

            let queries = query_string.split("&").collect::<Vec<&str>>();
            if queries.len() == 0 {
                return Err(());
            }
            let mut punumber = "";
            let mut isnumber = "";
            let mut has_punumber = false;
            let mut has_isnumber = false;

            for query in queries {
                let values = query.split("=").collect::<Vec<&str>>();
                if values[0] == "punumber" {
                    has_punumber = true;
                    punumber = values[1];
                }
                else if values[0] == "isnumber" {
                    has_isnumber = true;
                    isnumber = values[1];
                }
                if has_punumber && has_isnumber {
                    return Ok((punumber.to_string(), isnumber.to_string()));
                }
            }
        },
        _ => {
            // No query
            return Err(());
        }
    };
    return Err(());
}

pub fn get_download_source() -> DownloadSource {
    println!("{}", TermStyle("Select Download Source").bold().yellow());
    let source_options = vec!["Sci-Hub (Recommended, Fast)", "LibGen (Slow)"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&source_options)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();
    match selection {
        Some(index) => {
            match index {
                0 => {
                    return DownloadSource::SCIHUB;
                },
                1 => {
                    return DownloadSource::LIBGEN;
                }
                _ => {
                    panic!("Shouldn't reach here.")
                }
            };
        },
        None => panic!("You did not select anything")
    }
}

pub async fn get_record_store(client: &reqwest::Client, journal_url: &str, ieee_punumber: &str, ieee_isnumber: &str) -> Result<RecordStore, Box<dyn std::error::Error>> {
    // let journal_url = "https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=8802299&punumber=8014";
    // "https://ieeexplore.ieee.org/rest/search/pub/8014/issue/8802299/toc"
    let journal_toc_url = format!("https://ieeexplore.ieee.org/rest/search/pub/{}/issue/{}/toc", ieee_punumber, ieee_isnumber);

    let mut request_json_data = HashMap::new();
    request_json_data.insert("isnumber", ieee_isnumber);
    request_json_data.insert("punumber", ieee_punumber);
    request_json_data.insert("sortType", "vol-only-seq");

    let response = client
        .post(journal_toc_url)
        .header("Accept", "application/json, text/plain, */*")
        .header("Content-Type", "application/json")
        .header("Host", "ieeexplore.ieee.org")
        .header("Origin", "https://ieeexplore.ieee.org")
        .header("Referer", journal_url)
        .json(&request_json_data)
        .send()
        .await?;
    let response_raw_data = response.text().await?;
    let response_json_data : Value = serde_json::from_str(&response_raw_data)?;
    let records_string = format!("{{\"records\":{}}}", response_json_data["records"]);
    let record_store: RecordStore = serde_json::from_str(&records_string)?;
    Ok(record_store)
}

pub async fn download_documents(term: &Term, client: &reqwest::Client, download_source: &DownloadSource, record_store: &RecordStore, temp_directory: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    for (index, record) in record_store.records.iter().enumerate() {
        println!("{}", TermStyle(format!("[{}/{}]", index + 1, record_store.records.len())).bold());
        let status = get_download_url(&client, &download_source, &temp_directory, &index, record).await?;
        if !status {
            println!("{}", TermStyle(format!("{} Failed!", Emoji("❌", ":("))).bold().red());
        }
        utils::print_divider(&term);
        if index > 0 && index % 9 == 0 {
            for i in 0..60 {
                println!("To avoid any server captchas,\nwe must wait for {} seconds.",  60 - i);
                thread::sleep(time::Duration::from_millis(1000));
                term.clear_last_lines(2)?;
            }
        }
    }
    Ok(())
}

pub async fn get_download_url(client: &reqwest::Client, download_source: &DownloadSource, temp_directory: &std::path::Path, index: &usize, record: &Value) -> Result<bool, Box<dyn std::error::Error>> {
    let title = &record["articleTitle"].to_string();
    let access_type = &format_json_string(&record["accessType"]["type"].to_string()); // ephemera, locked
    println!("Downloading: {}", title);
    match access_type.as_str() {
        "ephemera" | "open-access" => {
            // Free
            let url_to_download = format_json_string(
                &format!("https://ieeexplore.ieee.org/ielx7/{}/{}/{}.pdf",
                &record["publicationNumber"].to_string(),
                &record["isNumber"].to_string(),
                &format!("0{}", record["articleNumber"].to_string()),
            ));
            println!("{:?} Url: {}", download_source, url_to_download);
            println!("{}", TermStyle("Fetching free document...").bold().yellow());
            download_url(&format!("{}.pdf", index), &url_to_download, temp_directory).await?;
        },
        "locked" => {
            let base_request_url = match download_source {
                DownloadSource::SCIHUB=>"https://sci-hub.do/",
                DownloadSource::LIBGEN=>"http://libgen.gs/scimag/ads.php?doi="
            };
        
            let selector = match download_source {
                DownloadSource::SCIHUB=>"iframe",
                DownloadSource::LIBGEN=>"a"
            };
        
            let attribute = match download_source {
                DownloadSource::SCIHUB=>"src",
                DownloadSource::LIBGEN=>"href"
            };
        
            let download_domain = match download_source {
                DownloadSource::SCIHUB=>"http:",
                DownloadSource::LIBGEN=>"http://libgen.gs"
            };
        
            let formatted_link = format_json_string(&format!("{}{}", base_request_url, record["doi"].to_string()));
            println!("{:?} Url: {}", download_source, formatted_link);
            println!("{}", TermStyle("Fetching locked document...").bold().yellow());
            let response = client.get(&formatted_link).send().await?;
            if !response.status().is_success() {
                return Ok(false);
            }
            let scihub_document = Html::parse_document(&response.text().await?);
            let link_selector = Selector::parse(selector).unwrap();
            let link_elements = scihub_document.select(&link_selector);
            let mut link_exists = false;
            for element in link_elements {
                link_exists = true;
                let attr_value = element.value().attr(attribute).unwrap();
                let url_to_download = match attr_value.chars().nth(0).unwrap() {
                    '/' => {
                        format!("{}{}", download_domain, attr_value)
                    }
                    _ => {
                        attr_value.to_string()
                    }
                };
                download_url(&format!("{}.pdf", index), &url_to_download, temp_directory).await?;
                break;
            }
            if !link_exists {
                return Ok(false);
            }
        }
        _ => {
            panic!("Unknown access type!");
        }
    }

    return Ok(true);
}

pub async fn download_url(file_name: &str, url: &str, directory: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let mut file_path = {
        File::create(directory.join(file_name))?
    };

    let total_size = response.content_length().ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    //pb.set_message(format!("Downloading {}", url));

    // Download chunks
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file_path.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("{}", TermStyle(format!("{} Done!", Emoji("✅", ":)"))).bold().green()));
    Ok(())
}

pub fn format_json_string(source_string: &str) -> String{
    str::replace(source_string, "\"", "")
}

pub fn clean_up () -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}