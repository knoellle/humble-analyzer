use std::{fs, path::Path};

#[derive(Debug)]
struct Item {
    name: Option<String>,
    steam_app_id: Option<u32>,
    key: Option<String>,
}

impl From<&json::JsonValue> for Item {
    fn from(value: &json::JsonValue) -> Self {
        Item {
            name: value["human_name"].as_str().map(|str| str.to_string()),
            steam_app_id: value["steam_app_id"].as_u32(),
            key: value["redeemed_key_val"]
                .as_str()
                .map(|str| str.to_string()),
        }
    }
}

impl Item {
    fn to_tsv(&self) {
        let url = self.steam_app_id.map_or("".to_string(), |id| {
            format!("https://store.steampowered.com/app/{}", id)
        });
        let redeemed = self.key.as_ref().map_or("0", |_| "1");
        println!("{}\t{}\t{}", self.name.as_ref().unwrap(), url, redeemed);
    }
}

fn read_file<P>(path: P) -> anyhow::Result<Vec<Item>>
where
    P: AsRef<Path>,
{
    Ok(json::parse(&fs::read_to_string(path)?)?
        .entries()
        .flat_map(|(_key, entry)| {
            entry["tpkd_dict"]["all_tpks"]
                .members()
                .map(|tpks| Item::from(tpks))
        })
        .collect())
}

fn main() -> anyhow::Result<()> {
    let files = std::path::Path::new("data")
        .join("orders")
        .read_dir()?
        .filter_map(|dir_item| dir_item.ok())
        .filter_map(|dir_item| {
            if dir_item.file_type().ok()?.is_file() && dir_item.path().extension()? == "json" {
                Some(dir_item.path())
            } else {
                None
            }
        });
    let mut items = files
        .flat_map(|path| read_file(path).unwrap())
        .filter(|item| item.name.is_some())
        .collect::<Vec<_>>();

    items.sort_by_key(|item| item.name.clone().unwrap_or("".to_string()));

    // println!("{:#?}", items);
    // println!("{} items", items.len());
    items
        .iter()
        // .filter(|item| item.key.is_none())
        .for_each(|item| item.to_tsv());
    Ok(())
}
