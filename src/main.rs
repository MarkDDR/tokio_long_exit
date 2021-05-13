use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PokemonStatResponse {
    stats: Vec<PokemonStat>,
    name: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct PokemonStat {
    base_stat: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://pokeapi.co/api/v2/pokemon/";
    let client = reqwest::Client::new();
    let mut handles = vec![];
    // TODO change to 251
    for id in 1..=251 {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            Result::<_, anyhow::Error>::Ok(
                client
                    .get(format!("{}{}", url, id))
                    .send()
                    .await?
                    .json::<PokemonStatResponse>()
                    .await?,
            )
        });
        handles.push(handle);
    }
    println!("all spawned");

    let mut pokemon_stats = vec![];
    for handle in handles {
        pokemon_stats.push(handle.await??);
    }

    let bst: Vec<_> = pokemon_stats
        .iter()
        .map(|p| (&p.name, p.stats.iter().map(|s| s.base_stat).sum::<i32>()))
        .collect();
    let bst_cutoff = 580;

    let num_above_cutoff: Vec<_> = bst.iter().filter(|&&(_, s)| s >= bst_cutoff).collect();

    for p in num_above_cutoff.iter() {
        println!("{:?}", p);
    }

    println!(
        "{} pokemon above cutoff of {} out of {}",
        num_above_cutoff.len(),
        bst_cutoff,
        bst.len()
    );
    println!("{}", num_above_cutoff.len() as f32 / bst.len() as f32);

    Ok(())
}
