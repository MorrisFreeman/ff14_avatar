use ff14_avatar::fetch_avatar;

#[tokio::main]
async fn main() {
    let id = "31270495";
    let avatar = fetch_avatar(id).await;
    println!("ID: {}", avatar.id);
    println!("Name: {}", avatar.name);
    println!("ImageUrl: {}", avatar.image_url);
    println!("Jobs: {:?}", avatar.jobs);
    println!("Achievements: {:?}", avatar.achievements);
    println!("Minions: {:?}", avatar.minions);
    println!("Mounts: {:?}", avatar.mounts);
}
