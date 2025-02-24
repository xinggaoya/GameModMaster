use crate::{AppError, Trainer};
use scraper::{Html, Selector};

pub fn parse_trainer_list(html: &str) -> Result<Vec<Trainer>, AppError> {
    let document = Html::parse_document(html);
    let article_selector = Selector::parse("article.post-standard")
        .map_err(|e| AppError::ParseError(e.to_string()))?;

    let mut trainers = Vec::new();

    for article in document.select(&article_selector) {
        // 提取标题和链接
        if let Some(title_element) = article
            .select(&Selector::parse("h2.post-title a").unwrap())
            .next()
        {
            let name = title_element.text().collect::<String>().trim().to_string();
            let href = title_element.value().attr("href").unwrap_or_default();

            // 从 URL 中提取 ID
            let id = href
                .trim_end_matches('/')
                .split('/')
                .last()
                .unwrap_or_default()
                .to_string();

            // 提取缩略图
            let thumbnail = article
                .select(&Selector::parse(".post-details-thumb img").unwrap())
                .next()
                .and_then(|img| img.value().attr("src"))
                .unwrap_or_default()
                .to_string();

            // 提取版本信息和游戏版本
            let entry_text = article
                .select(&Selector::parse(".entry").unwrap())
                .next()
                .map(|content| content.text().collect::<String>())
                .unwrap_or_default();

            let mut version = String::new();
            let mut game_version = String::new();

            // 解析版本信息
            if let Some(options_text) = entry_text.split('·').next() {
                version = options_text.trim().to_string();
            }

            // 解析游戏版本
            if let Some(game_version_text) = entry_text.split('·').nth(1) {
                if game_version_text.contains("Game Version:") {
                    game_version = game_version_text
                        .replace("Game Version:", "")
                        .trim()
                        .to_string();
                }
            }

            // 提取更新日期
            let day = article
                .select(&Selector::parse(".post-details-day").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let month = article
                .select(&Selector::parse(".post-details-month").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let year = article
                .select(&Selector::parse(".post-details-year").unwrap())
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let last_update = format!("{} {} {}", day, month, year);

            trainers.push(Trainer {
                id,
                name,
                version,
                game_version,
                download_url: String::new(), // 这个需要从详情页获取
                description: entry_text,
                thumbnail,
                download_count: 0, // 这个需要从详情页获取
                last_update,
            });
        }
    }

    Ok(trainers)
}

pub fn parse_trainer_detail(html: &str) -> Result<Trainer, AppError> {
    let document = Html::parse_document(html);

    // 提取标题
    let title = document
        .select(&Selector::parse("h1.post-title").unwrap())
        .next()
        .map(|el| el.text().collect::<String>())
        .ok_or_else(|| AppError::ParseError("Title not found".to_string()))?
        .trim()
        .to_string();

    // 提取缩略图
    let thumbnail = document
        .select(&Selector::parse(".entry img.aligncenter").unwrap())
        .next()
        .and_then(|img| img.value().attr("src"))
        .unwrap_or_default()
        .to_string();

    // 提取主要内容
    let entry_text = document
        .select(&Selector::parse(".entry").unwrap())
        .next()
        .map(|content| {
            // 移除广告div
            let html = content.inner_html();
            let cleaned_html = html
                .lines()
                .filter(|line| {
                    !line.contains("fling-before-content") && !line.contains("fling-after-content")
                })
                .collect::<Vec<_>>()
                .join("\n");
            Html::parse_fragment(&cleaned_html)
                .root_element()
                .text()
                .collect::<String>()
        })
        .unwrap_or_default();

    // 解析第一行信息
    let first_line = entry_text
        .lines()
        .find(|line| line.contains("Options") && line.contains("Game Version:"))
        .unwrap_or("")
        .trim()
        .to_string();

    let parts: Vec<String> = first_line
        .split('·')
        .map(|s| s.trim().to_string())
        .collect();

    // 提取版本信息（Options数量）
    let version = parts.get(0).cloned().unwrap_or_default();

    // 提取游戏版本
    let game_version = parts
        .get(1)
        .map(|s| s.replace("Game Version:", "").trim().to_string())
        .unwrap_or_default();

    // 提取最后更新时间
    let last_update = parts
        .get(2)
        .map(|s| s.replace("Last Updated:", "").trim().to_string())
        .unwrap_or_default();

    // 提取功能列表作为描述
    let mut description = String::new();
    let mut in_options = false;
    for line in entry_text.lines() {
        let line = line.trim();
        if line.contains("Options") && !in_options {
            in_options = true;
            continue;
        }
        if in_options && !line.is_empty() {
            if line.starts_with("Download") {
                break;
            }
            description.push_str(line);
            description.push('\n');
        }
    }

    // 提取下载次数
    let download_count = document
        .select(&Selector::parse(".attachment-downloads").unwrap())
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default()
        .parse()
        .unwrap_or(0);

    // 提取下载链接
    let download_url = document
        .select(&Selector::parse(".attachment-link").unwrap())
        .next()
        .and_then(|a| a.value().attr("href"))
        .ok_or_else(|| AppError::ParseError("Download link not found".to_string()))?
        .to_string();

    // 从URL中提取ID
    let id = document
        .select(&Selector::parse("link[rel='canonical']").unwrap())
        .next()
        .and_then(|link| link.value().attr("href"))
        .map(|url| {
            url.trim_end_matches('/')
                .split('/')
                .last()
                .unwrap_or_default()
                .replace("-trainer", "")
                .to_string()
        })
        .unwrap_or_else(|| {
            title
                .to_lowercase()
                .replace(" trainer", "")
                .replace(' ', "-")
        });

    Ok(Trainer {
        id,
        name: title,
        version,
        game_version,
        download_url,
        description,
        thumbnail,
        download_count,
        last_update,
    })
}
