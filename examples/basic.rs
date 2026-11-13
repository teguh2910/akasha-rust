use akasha::{Akasha, Language, LeaderboardOptions};

#[tokio::main]
async fn main() -> akasha::Result<()> {
    let uid = 901211014;
    let api = Akasha::new(Language::ChineseSimplified)?;

    let characters = api.get_calculations_for_user(uid).await?;
    for character in characters {
        let Some(calc) = character.calculations.first() else {
            continue;
        };

        println!("Character: {}", character.name);
        println!("Weapon: {}", calc.weapon.name);
        println!(
            "Top {:.2}% ({}/{})",
            calc.top_percent(),
            calc.ranking,
            calc.out_of
        );
        println!("Damage: {}", calc.result.round());
        println!();

        println!("Leaderboard top 3:");
        let mut boards =
            api.get_leaderboards(calc.id, LeaderboardOptions::new().max_page(1).page_size(3));
        while let Some(board) = boards.next().await? {
            println!(
                "{}. {} | Damage: {}",
                board.rank,
                board.owner.nickname,
                board.calculation.result.round()
            );
        }

        println!(
            "Leaderboard URL: https://akasha.cv/leaderboards/{}",
            calc.id
        );
        println!("{}", "=".repeat(50));
    }

    Ok(())
}
