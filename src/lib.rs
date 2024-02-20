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

/// A FF14 character avatar.
#[derive(Debug)]
pub struct FF14Avatar {
    pub id: String,
    pub name: String,
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

    FF14Avatar {
        id: id.to_string(),
        name,
        image_url
    }
}
