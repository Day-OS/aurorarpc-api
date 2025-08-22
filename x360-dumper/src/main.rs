use roxmltree::Document;
use anyhow::{anyhow};
use x360connect_global::schm_game::{Category, Images, SchmGame};
use rand::Rng;
use std::{env, time::Duration};
use tokio::time::sleep;

fn get_first_child<'a>(node: roxmltree::Node<'a, 'a>, tag: &'a str) -> anyhow::Result<roxmltree::Node<'a, 'a>> {
    let result = node.children().find(|node | node.tag_name().name() == tag)
            .ok_or(anyhow!("Badly formatted xml"))?;
    Ok(result)
}

fn get_first_child_text<'a>(node: roxmltree::Node<'a, 'a>, tag: &'a str) -> anyhow::Result<&'a str> {
    let result = get_first_child(node, tag)?.text().ok_or(anyhow!("No text inside of {:?}", tag))?;
    Ok(result)
}

pub fn parse(input: &str) -> anyhow::Result<(u16, u16, Vec<SchmGame>)> {
    let binding = Document::parse(input)?;
    let doc: roxmltree::Node<'_, '_> = binding.root_element();
    
    let total_items = get_first_child_text(doc, "totalItems")?
            .parse::<u16>()?;

    let num_items = get_first_child_text(doc, "numItems")?
            .parse::<u16>()?;

    let entries = doc.children().filter(|n| n.tag_name().name() == "entry");
    let mut entries_in: Vec<SchmGame> = vec![];
    
    for entry in entries{

        let fulltitle = get_first_child_text(entry, "title").map(|s| s.to_owned()).ok();
        let mut title_id = None;
        let mut description = None;
        let mut reduced_title = None;
        let mut release_date = None;
        let mut developer = None;
        let mut publisher = None;
        let mut rating_aggregate = None;
    
        if let Ok(md) = get_first_child(entry, "media"){
            title_id = get_first_child_text(md, "titleId").map(|s| s.parse::<i64>().unwrap()).ok();
            description = get_first_child_text(md, "reducedDescription").map(|s| s.to_owned()).ok();
            reduced_title = get_first_child_text(md, "reducedTitle").map(|s| s.to_owned()).ok();
            release_date = get_first_child_text(md, "releaseDate").map(|s| s.to_owned()).ok();
            developer = get_first_child_text(md, "developer").map(|s| s.to_owned()).ok();
            publisher = get_first_child_text(md, "publisher").map(|s| s.to_owned()).ok();
            rating_aggregate = get_first_child_text(md, "ratingAggregate").map(|s| s.to_owned()).ok();
        }




        let mut categories = None;
        if let Ok(cat_list) = get_first_child(entry, "categories"){
            let mut list = vec![];
            for cat in cat_list.children(){
                list.push(Category{
                    categoryid: get_first_child_text(cat, "categoryId")?.to_owned(),
                    system: get_first_child_text(cat, "system")?.to_owned(),
                    name: get_first_child_text(cat, "name")?.to_owned(),
                });
            }
            categories = Some(list);
        };

        let mut images = None;
        
        if let Ok(img) = get_first_child(entry, "images"){
            let mut boxart = None;
            let mut banner = None;
            let mut icon = None;

            for image in img.children(){
                let file_url = get_first_child_text(image, "fileUrl")?.to_owned();
                let r_type = get_first_child_text(image, "relationshipType")?.parse::<u8>()?;
                match r_type {
                    23 => icon = Some(file_url),
                    33 => boxart = Some(file_url),
                    27 => banner = Some(file_url),
                    _ => {}
                };
            }

            images = Some(Images{ 
                boxart, 
                icon,
                banner
            });
        };



        let scheme = SchmGame{
            fulltitle,
            title_id,
            description,
            reduced_title,
            release_date,
            developer,
            publisher,
            rating_aggregate,
            categories,
            images,
        };
        entries_in.push(scheme);
    }
    Ok((total_items, num_items, entries_in))
}

pub async fn get_list(url: &str, char: &str, page_number: u16) -> anyhow::Result<String>{
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;

    let url = format!(
        "{url}?methodName=FindGames&Names=Locale&Values=en-US&Names=LegalLocale&Values=en-US&Names=Store&Values=1&Names=PageSize&Values=100&Names=PageNum&Values={page_number}&Names=DetailView&Values=3&Names=AvatarBodyTypes&Values=3&Names=AvatarBodyTypes&Values=1&Names=UserTypes&Values=2&Names=UserTypes&Values=3&Names=MediaTypes&Values=1&Names=MediaTypes&Values=21&Names=MediaTypes&Values=23&Names=ImageFormats&Values=4&Names=ImageFormats&Values=5&Names=ImageSizes&Values=14&Names=ImageSizes&Values=15&Names=ImageSizes&Values=23&Names=OrderBy&Values=1&Names=OrderDirection&Values=1&Names=OfferFilterLevel&Values=2&Names=CategoryIds&Values=3027&Names=TitleFilters&Values={char}",
    );

    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let chars_to_search = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", 
        "9", "a", "b", "c", "d", "e", "f", "g", "h", 
        "i","j", "k", "l", "m", "n", "o", "p", "q",
        "r", "s", "t", "u", "v", "w", "x", "y", "z"
    ];
    // for "scared of the devil" reasons, the URL that the data is gathered is not included in the source
    let url_env = env::var("API_URL")?;

    let mut all_games = vec![];

    for char in chars_to_search{
        let mut quantity_to_fill: u16 = 1;
        let mut page_number: u16 = 1;
        let mut first_time = true;

        while quantity_to_fill > 0 {
            let input = get_list(&url_env, char, page_number).await?;
            let (total_items, num_items, mut games) = parse(&input)?;
            if first_time{
                quantity_to_fill = total_items;
                first_time = false;
            }
            quantity_to_fill -= num_items;
            page_number += 1;
            all_games.append(&mut games);
            println!("Filling CHAR: {char} - TOTAL: {total_items}")
        }
        let list = serde_json::to_string(&all_games)?;
        std::fs::write(format!("games.json"), &list)?;

        let wait_secs = rand::rng().random_range(1..=10);
        for i in (1..=wait_secs).rev() {
            println!("Waiting... {} seconds remaining", i);
            sleep(Duration::from_secs(1)).await;
        }
    }



    Ok(())
}
