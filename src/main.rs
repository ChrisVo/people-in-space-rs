#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub message: String,
    pub number: i64,
    pub people: Vec<People>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct People {
    pub craft: String,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://api.open-notify.org/astros.json").await?;
    let p: Root = resp.json().await?;
    println!("\n👨‍🚀 There are {} people in space 🧑‍🚀\n", p.number);
    for people in p.people.iter() {
        println!("- {} on the {}", people.name, people.craft)
    }
    Ok(())
}
