use crate::models::{NyaaCategory, NyaaListItem, NyaaTorrent};
use regex::Regex;
use scraper::{Html, Selector};

pub fn parse_nyaa_list(html: String, url: &str) -> Vec<NyaaListItem> {
    let doc = Html::parse_document(&html);
    let selector = Selector::parse("tbody tr").unwrap();
    let sel: scraper::html::Select = doc.select(&selector);
    let mut nyaa_list: Vec<NyaaListItem> = Vec::new();
    for element in sel {
        nyaa_list.push(parse_nyaa_list_item(element, &url));
    }
    nyaa_list
}

pub fn parse_nyaa_list_item(element: scraper::ElementRef, url: &str) -> NyaaListItem {
    let re: Regex = Regex::new(r"magnet:\?xt=urn:btih:(.*?)&dn=").unwrap();
    let title_selector = Selector::parse("a:not([class=\"comments\"])").unwrap();
    let stuff_selector = Selector::parse("td").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let mut nyaa_list_item: NyaaListItem = NyaaListItem {
        ..Default::default()
    };
    match element.value().attr("class").unwrap() {
        "default" => nyaa_list_item.torrent_type = Some("Normal".to_string()),
        "danger" => nyaa_list_item.torrent_type = Some("Remake".to_string()),
        "success" => nyaa_list_item.torrent_type = Some("Trusted".to_string()),
        _ => {}
    }
    for (i, stuff) in element.select(&stuff_selector).enumerate() {
        match i {
            0 => {
                let category = stuff.select(&link_selector).next().unwrap();
                nyaa_list_item.category = Some(NyaaCategory {
                    name: Some(category.value().attr("title").unwrap().to_string()),
                    id: Some(
                        category
                            .value()
                            .attr("href")
                            .unwrap()
                            .to_string()
                            .replace("/?c=", ""),
                    ),
                })
            }
            1 => {
                nyaa_list_item.title = Some(
                    stuff
                        .select(&title_selector)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<String>(),
                );
            }
            2 => {
                for (i, link) in stuff.select(&link_selector).enumerate() {
                    match i {
                        0 => {
                            nyaa_list_item.torrent =
                                Some(format!("{}{}", &url, &link.value().attr("href").unwrap()))
                        }
                        1 => {
                            let magnet_link: &str = link.value().attr("href").unwrap();
                            nyaa_list_item.infohash =
                                Some(re.captures(magnet_link).unwrap()[1].to_string());
                            nyaa_list_item.magnet = Some(magnet_link.to_string())
                        }
                        _ => {}
                    }
                }
            }
            3 => nyaa_list_item.size = Some(stuff.text().collect::<String>()),
            4 => nyaa_list_item.date = Some(stuff.text().collect::<String>()),
            5 => nyaa_list_item.seeders = Some(stuff.text().collect::<String>()),
            6 => nyaa_list_item.leechers = Some(stuff.text().collect::<String>()),
            7 => nyaa_list_item.downloads = Some(stuff.text().collect::<String>()),
            _ => {}
        }
    }
    nyaa_list_item
}

pub fn parse_nyaa_torrent(html: String, url: &str) -> NyaaTorrent {
    let mut nyaatorrent: NyaaTorrent = NyaaTorrent {
        ..Default::default()
    };
    let doc: Html = Html::parse_document(&html);
    let title_selector: Selector = Selector::parse("h3.panel-title").unwrap();
    let data_selector: Selector = Selector::parse("div.col-md-5").unwrap();
    let download_selector: Selector = Selector::parse("div.panel-footer.clearfix a").unwrap();
    let comment_selector: Selector = Selector::parse("a h3.panel-title").unwrap();
    let link_selector: Selector = Selector::parse("a").unwrap();
    let panel_selector: Selector = Selector::parse("div.container>div.panel:first-child").unwrap();
    let mut downloads = doc.select(&download_selector);
    let panel_class = doc
        .select(&panel_selector)
        .next()
        .unwrap()
        .value()
        .attr("class")
        .unwrap();
    match panel_class {
        "panel panel-default" => nyaatorrent.torrent_type = Some("Normal".to_string()),
        "panel panel-danger" => nyaatorrent.torrent_type = Some("Remake".to_string()),
        "panel panel-success" => nyaatorrent.torrent_type = Some("Trusted".to_string()),
        _ => {}
    }
    nyaatorrent.torrent = Some(format!(
        "{}{}",
        &url,
        &downloads.next().unwrap().value().attr("href").unwrap()
    ));
    nyaatorrent.magnet = Some(
        downloads
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_string(),
    );
    nyaatorrent.comments = Some(
        doc.select(&comment_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .replace("\t", "")
            .replace("\n", "")
            .replace("Comments - ", ""),
    );
    nyaatorrent.title = Some(
        doc.select(&title_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .replace("\t", "")
            .replace("\n", ""),
    );

    for (i, data) in doc.select(&data_selector).enumerate() {
        match i {
            0 => {
                let mut selected_links = data.select(&link_selector);
                selected_links.next().unwrap();
                nyaatorrent.category = Some(NyaaCategory {
                    name: Some(
                        data.text()
                            .collect::<String>()
                            .replace("\t", "")
                            .replace("\n", ""),
                    ),
                    id: Some(
                        selected_links
                            .next()
                            .unwrap()
                            .value()
                            .attr("href")
                            .unwrap()
                            .replace("/?c=", ""),
                    ),
                })
            } // Category
            1 => {
                nyaatorrent.date = Some(
                    data.text()
                        .collect::<String>()
                        .replace("\t", "")
                        .replace("\n", ""),
                )
            } // Date
            2 => {
                nyaatorrent.submitter = Some(
                    data.text()
                        .collect::<String>()
                        .replace("\t", "")
                        .replace("\n", ""),
                )
            } // Submitter
            3 => {
                nyaatorrent.seeders = Some(
                    data.text()
                        .collect::<String>()
                        .replace("\t", "")
                        .replace("\n", ""),
                )
            } // Seeders
            4 => {
                nyaatorrent.info = Some(
                    data.text()
                        .collect::<String>()
                        .replace("\t", "")
                        .replace("\n", ""),
                )
            } // Info
            5 => {
                nyaatorrent.leechers = Some(
                    data.text()
                        .collect::<String>()
                        .replace("\t", "")
                        .replace("\n", ""),
                )
            } // Leechers
            6 => {
                nyaatorrent.size = Some(
                    data.text()
                        .collect::<String>()
                        .replace("\t", "")
                        .replace("\n", ""),
                )
            } // Size
            7 => {
                nyaatorrent.downloads = Some(
                    data.text()
                        .collect::<String>()
                        .replace("\t", "")
                        .replace("\n", ""),
                )
            } // Downloads
            8 => {
                nyaatorrent.infohash = Some(
                    data.text()
                        .collect::<String>()
                        .replace("\t", "")
                        .replace("\n", ""),
                )
            } // Infohash
            _ => {}
        }
    }
    nyaatorrent
}
