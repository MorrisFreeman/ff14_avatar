use ff14_avatar::fetch_avatar;

#[tokio::main]
async fn main() {
    let id = "YOUR";
    let avatar = fetch_avatar(id).await;
    println!("ID: {}", avatar.id);
    println!("Name: {}", avatar.name);
    println!("ImageUrl: {}", avatar.image_url);
}
