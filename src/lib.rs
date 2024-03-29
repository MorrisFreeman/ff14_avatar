use std::hash;

/// This is the main file for the library.
///
/// The library is responsible for fetching FF14 character avatars.
///
/// # Examples
///
/// ```
/// use ff14_avatar::fetch_avatars;
///
/// #[tokio::main]
/// async fn main() {
///     let id = "YOUR";
///     let avatar = fetch_avatar(id).await;
///     println!("ID: {}", avatar.id);
///     println!("Name: {}", avatar.name);
///     println!("ImageUrl: {}", avatar.image_url);
/// }
/// ```
use scraper::Selector;
use regex::Regex;
use chrono::{TimeZone, DateTime, Utc};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use hex;

/// A FF14 character avatar.
#[derive(Serialize, Deserialize, Debug)]
pub struct FF14Avatar {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub jobs: Jobs,
    pub achievements: Achievements,
    pub minions: Minions,
    pub mounts: Mounts,
    pub fetched_at: DateTime<Utc>,
    pub image_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    pub name: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mounts {
    pub items: Vec<Mount>,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Minions {
    pub items: Vec<Minion>,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Minion {
    pub name: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Achievements {
    pub items: Vec<Achievement>,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Achievement {
    pub title: String,
    pub date: DateTime<Utc>,
    pub image_url: String,
}

type Jobs = Vec<Job>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub name: String,
    pub level: String,
    pub image_url: String,
}

/// Fetches FF14 character avatars.
///
/// # Arguments
///
/// * `ids` - A vector of FF14 character IDs.
pub async fn fetch_avatars(ids: Vec<String>) -> Vec<FF14Avatar> {
    let mut avatars = Vec::new();
    for id in ids {
        let avatar = fetch_avatar(&id).await;
        avatars.push(avatar);
    }
    avatars
}

/// Fetches a FF14 character avatar.
///
/// # Arguments
///
/// * `id` - A FF14 character ID.
pub async fn fetch_avatar(id: &str) -> FF14Avatar {
    let base_url = "https://jp.finalfantasyxiv.com/lodestone/character";
    let url = format!("{}/{}/", base_url, id);
    let html = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let doc: scraper::Html = scraper::Html::parse_document(&html);

    // Get the character name
    let name_sel = Selector::parse(".frame__chara__name").unwrap();
    let node = doc.select(&name_sel).next().unwrap();
    let name = node.text().collect::<Vec<_>>().join("");
    let name = name.trim().to_string();

    // Get the character image url
    let image_sel = Selector::parse(".character__detail__image a").unwrap();
    let node = doc.select(&image_sel).next().unwrap();
    let href = node.value().attr("href").unwrap();
    let image_url = href.to_string();
    let res = reqwest::get(&image_url).await.unwrap();
    let bytes = res.bytes().await.unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash_result = hasher.finalize();
    let image_hash = hex::encode(hash_result);

    // Get the character jobs
    let mut jobs: Vec<Job> = Vec::new();
    let job_sel = Selector::parse(".character__level__list li").unwrap();
    for node in doc.select(&job_sel) {
        let level = node.text().collect::<Vec<_>>().join("");
        let node = node.select(&Selector::parse("img").unwrap()).next().unwrap();
        let name = node.value().attr("data-tooltip").unwrap().to_string();
        let image_url = node.value().attr("src").unwrap().to_string();
        let job = Job {
            name,
            level,
            image_url,
        };

        jobs.push(job);
    }

    // Get the character achievements
    let url = format!("{}/{}/achievement/", base_url, id);
    let html = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let doc: scraper::Html = scraper::Html::parse_document(&html);
    let mut items: Vec<Achievement> = Vec::new();
    let achievement_sel = Selector::parse(".entry__achievement").unwrap();
    for node in doc.select(&achievement_sel) {
        let img_node = node.select(&Selector::parse("img").unwrap()).next().unwrap();
        let image_url = img_node.value().attr("src").unwrap().to_string();
        let date_node = node.select(&Selector::parse(".entry__activity__time script").unwrap()).next().unwrap();
        let date_script = date_node.text().collect::<Vec<_>>().join("");
        let re = Regex::new(r"ldst_strftime\((\d+), 'YMD'\)").unwrap();
        let timestamp = re.captures(&date_script).unwrap().get(1).unwrap().as_str().parse().unwrap(); // TODO: エラーになるかもしれない
        let date = Utc.timestamp_opt(timestamp, 0).unwrap();
        let title_node = node.select(&Selector::parse(".entry__activity__txt").unwrap()).next().unwrap();
        let title = title_node.text().collect::<Vec<_>>().join("");
        let achievement = Achievement {
            title,
            date,
            image_url,
        };

        items.push(achievement);
    }
    let count_sel = Selector::parse(".parts__total").unwrap();
    let count_str  = doc.select(&count_sel).next().unwrap().text().collect::<Vec<_>>().join("");
    let re = Regex::new(r"(\d+).*").unwrap();
    let count = re.captures(&count_str).unwrap().get(1).unwrap().as_str().parse().unwrap(); // TODO: エラーになるかもしれない

    let achievements = Achievements {
        items,
        count,
    };

    // Get the character minions
    let url = format!("{}/{}/minion/", base_url, id);
    let html = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let doc: scraper::Html = scraper::Html::parse_document(&html);
    let mut items: Vec<Minion> = Vec::new();
    let minion_sel = Selector::parse(".minion__list_icon").unwrap();
    for node in doc.select(&minion_sel) {
        let img_node = node.select(&Selector::parse("img").unwrap()).next().unwrap();
        let image_url = img_node.value().attr("src").unwrap().to_string();
        let minion = Minion {
            name: "".to_string(),
            image_url,
        };

        items.push(minion);
    }
    let count_sel = Selector::parse(".minion__sort__total span").unwrap();
    let count = doc.select(&count_sel).next().unwrap().text().collect::<Vec<_>>().join("").parse().unwrap();
    let minions = Minions {
        items,
        count,
    };

    // Get the character mounts
    let url = format!("{}/{}/mount/", base_url, id);
    let html = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let doc: scraper::Html = scraper::Html::parse_document(&html);
    let mut items: Vec<Mount> = Vec::new();
    let mount_sel = Selector::parse(".character__item_icon").unwrap();
    for node in doc.select(&mount_sel) {
        let img_node = node.select(&Selector::parse("img").unwrap()).next().unwrap();
        let image_url = img_node.value().attr("src").unwrap().to_string();
        let mount = Mount {
            name: "".to_string(),
            image_url,
        };

        items.push(mount);
    }
    let count_sel = Selector::parse(".minion__sort__total span").unwrap();
    let count = doc.select(&count_sel).next().unwrap().text().collect::<Vec<_>>().join("").parse().unwrap();
    let mounts = Mounts {
        items,
        count,
    };

    let fetched_at = Utc::now();

    FF14Avatar {
        id: id.to_string(),
        name,
        image_url,
        jobs,
        achievements,
        minions,
        mounts,
        fetched_at,
        image_hash,
    }
}
