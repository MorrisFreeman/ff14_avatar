use ff14_avatar::fetch_avatar;

#[tokio::main]
async fn main() {
    let id = "Your ID here.";
    let avatar = fetch_avatar(id).await;
    println!("ID: {}", avatar.id);
    println!("Name: {}", avatar.name);
    println!("ImageUrl: {}", avatar.image_url);
    println!("Jobs: {:?}", avatar.jobs);
    println!("Achievements: {:?}", avatar.achievements);
    println!("Minions: {:?}", avatar.minions);
    println!("Mounts: {:?}", avatar.mounts);
}
