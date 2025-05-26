use futures::{Stream, StreamExt};
use poise::serenity_prelude::{self as serenity};

use crate::{commands::helpers::truncate::truncate_string, Context, Error};

struct MapAutocomplete {
    name: String,
    id: String,
}

pub fn find_bsr(ctx: Context<'_>, input: &str) -> Option<String> {
    // copied and pasted twitch icon
    if let Some(stripped) = input.strip_prefix("!bsr ") {
        return Some(stripped.to_string());
    }
    // beatsaver link
    else if let Some(caps) = ctx.data().bsr_link_regex.captures(input) {
        return Some(caps["bsr"].to_string());
    }
    // just the code flat out
    else if let Some(caps) = ctx.data().hexstring_regex.captures(input.trim()) {
        return Some(caps[0].to_string());
    }

    None
}

pub async fn autocomplete_map<'a>(
    ctx: Context<'_>,
    input: &'a str,
) -> impl Stream<Item = serenity::AutocompleteChoice> + 'a {
    let maps: Vec<MapAutocomplete> = match find_bsr(ctx, input) {
        Some(bsr) => handle_code(bsr, ctx).await.unwrap(),
        None => handle_search(input.to_string(), ctx).await.unwrap(),
    };

    futures::stream::iter(maps)
        .map(move |map: MapAutocomplete| serenity::AutocompleteChoice::new(map.name, map.id))
}

async fn handle_code(bsr: String, ctx: Context<'_>) -> Result<Vec<MapAutocomplete>, Error> {
    let res = ctx.data().beatsaver_client.map(&bsr).await?;
    let uploader = format!(" [{}]", res.uploader.name);
    let name = format!(
        "{}{}",
        truncate_string(res.name.clone(), 100 - uploader.len(), "...".into()),
        uploader
    );

    Ok(vec![MapAutocomplete {
        name,
        id: bsr.clone(),
    }])
}

async fn handle_search(query: String, ctx: Context<'_>) -> Result<Vec<MapAutocomplete>, Error> {
    let search_results = ctx
        .data()
        .beatsaver_client
        .search_maps(&query)
        .await
        .unwrap()
        .docs;

    Ok(search_results
        .iter()
        .map(|res| {
            let uploader = format!(" [{}]", res.uploader.name);
            let name = format!(
                "{}{}",
                truncate_string(res.name.clone(), 100 - uploader.len(), "...".into()),
                uploader
            );

            MapAutocomplete {
                name,
                id: res.id.clone(),
            }
        })
        .collect())
}
