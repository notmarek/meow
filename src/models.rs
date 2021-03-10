use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NyaaCategory {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NyaaListItem {
    pub title: Option<String>,
    pub category: Option<NyaaCategory>,
    pub torrent: Option<String>,
    pub magnet: Option<String>,
    pub size: Option<String>,
    pub date: Option<String>,
    pub seeders: Option<String>,
    pub leechers: Option<String>,
    pub downloads: Option<String>,
    pub torrent_type: Option<String>,
    pub infohash: Option<String>,
}

impl Default for NyaaListItem {
    fn default() -> NyaaListItem {
        NyaaListItem {
            title: Some("".to_string()),
            category: Some(NyaaCategory {
                id: Some("".to_string()),
                name: Some("".to_string()),
            }),
            torrent: Some("".to_string()),
            magnet: Some("".to_string()),
            size: Some("".to_string()),
            date: Some("".to_string()),
            seeders: Some("".to_string()),
            leechers: Some("".to_string()),
            downloads: Some("".to_string()),
            torrent_type: Some("".to_string()),
            infohash: Some("".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NyaaTorrent {
    pub title: Option<String>,
    pub category: Option<NyaaCategory>,
    pub comments: Option<String>,
    pub info: Option<String>,
    pub submitter: Option<String>,
    pub torrent: Option<String>,
    pub magnet: Option<String>,
    pub size: Option<String>,
    pub date: Option<String>,
    pub seeders: Option<String>,
    pub leechers: Option<String>,
    pub downloads: Option<String>,
    pub torrent_type: Option<String>,
    pub infohash: Option<String>,
}

impl Default for NyaaTorrent {
    fn default() -> NyaaTorrent {
        NyaaTorrent {
            title: Some("".to_string()),
            category: Some(NyaaCategory {
                id: Some("".to_string()),
                name: Some("".to_string()),
            }),
            torrent: Some("".to_string()),
            comments: Some("".to_string()),
            info: Some("".to_string()),
            submitter: Some("".to_string()),
            magnet: Some("".to_string()),
            size: Some("".to_string()),
            date: Some("".to_string()),
            seeders: Some("".to_string()),
            leechers: Some("".to_string()),
            downloads: Some("".to_string()),
            torrent_type: Some("".to_string()),
            infohash: Some("".to_string()),
        }
    }
}
