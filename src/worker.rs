use std::{ops::Range, time::Duration};

use anyhow::anyhow;
use chrono::{Local, Timelike};
use rand::{seq::SliceRandom, Rng};
use tracing::info;

use crate::{
    config::Config,
    constants::{TASKS, UPGRADES},
    not_pixel::NotPixel,
    telegram::client::Client,
};

pub async fn worker_execution(config: Config, not_pixel: &mut NotPixel) -> anyhow::Result<()> {
    let me = not_pixel.get_me().await?;
    let mining_status = not_pixel.get_mining_status().await?;
    let need_claim = config.need_claim.unwrap_or(false);

    if let Some(paint) = config.paint {
        info!(
            "Remaining charges for {}({}) - {}",
            me.first_name, me.id, mining_status.charges
        );

        for _ in 0..mining_status.charges {
            let coords = paint.coords.unwrap_or(((0, 1000), (0, 1000)));

            let x = rand::thread_rng().gen_range(coords.0 .0..coords.0 .1);
            let y = rand::thread_rng().gen_range(coords.1 .0..coords.1 .1);

            let color = paint
                .colors
                .choose(&mut rand::thread_rng())
                .ok_or(anyhow!("Random choice color failed"))?
                .clone();

            let response = not_pixel.paint(x, y, color).await?;
            info!(
                "Print status for {}({}) - {:?}",
                me.first_name, me.id, response
            );

            tokio::time::sleep(Duration::from_millis(paint.delay)).await;
        }
    }

    if let Some(tasks) = config.tasks {
        if !tasks.needed.iter().all(|task| TASKS.contains(task)) {
            return Err(anyhow!("Any task is not founded"));
        }

        let needed = tasks.needed;

        for task in needed {
            if task.starts_with("channel") {
                let channel_join = tasks.channel_join.unwrap_or(false);

                if channel_join {
                    let channel = task.replace("channel:", "");

                    if !not_pixel.telegram().get_channel(channel.clone()).await? {
                        info!("Channel joining is required. Joining to {}", channel);

                        not_pixel.telegram().join_channel(channel).await?;
                    }
                }
            }

            not_pixel.solve_task(task).await?;
        }
    }

    if need_claim {
        not_pixel.claim().await?;
    }

    // if let Some(boosts) = config.boosts {
    //     let current_boosts = not_pixel.get_mining_status().await?.boosts;

    //     let exclude = boosts.exclude.unwrap_or(vec![]);
    //     let boosts_names = vec!["paintReward", "reChargeSpeed", "energyLimit"]
    //         .iter()
    //         .filter(|x| !exclude.contains(&x.to_string()))
    //         .cloned()
    //         .collect::<Vec<_>>();

    //     let needed_boosts = boosts_names
    //         .iter()
    //         .filter_map(|s| {
    //             UPGRADES.get(*s).map(|boost_amounts| {
    //                 (
    //                     s.to_string(),
    //                     boost_amounts[(current_boosts[*s].as_i64().unwrap() + 1 - 2) as usize],
    //                 )
    //             })
    //         })
    //         .collect::<Vec<_>>();

    //     let needed_amount = needed_boosts.iter().map(|a| a.1.clone()).sum::<i32>() as i64;
    //     let minimal_amount_to_save = boosts.minimal_amount_to_save;

    //     if needed_amount > minimal_amount_to_save {
    //         info!("{needed_amount} > {minimal_amount_to_save} - skipping upgrade");
    //     } else {
    //         not_pixel.
    //     }
    // }

    Ok(())
}

pub async fn create_worker(
    config: Config,
    client: Client,
    user_agent: String,
    proxy: String,
) -> anyhow::Result<()> {
    let mut not_pixel = NotPixel::new(
        client.clone(),
        user_agent,
        Some(proxy),
        config.ref_id.clone(),
    )
    .await?;

    loop {
        let start_range = Range {
            start: config.time_to_start.0.clone(),
            end: config.time_to_start.1.clone() + 1,
        }
        .collect::<Vec<_>>();

        let start_time = start_range[rand::thread_rng().gen_range(0..start_range.len()) as usize];
        let date = Local::now();

        if date.hour() <= start_time {
            let duration = Duration::from_secs(((start_time - date.hour()) * 60 * 60).into());

            info!(
                "current time does not included in time_to_start range. Waiting... {}",
                duration.as_secs()
            );

            tokio::time::sleep(duration).await;
        }

        worker_execution(config.clone(), &mut not_pixel).await?;
    }

    Ok(())
}
