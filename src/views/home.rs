use super::filters;
use crate::auth::Oauth2User;
use crate::db::article::{get_all_active, get_all_archived, get_all_starred, Article};
use crate::db::user::get_connected_user;
use crate::errors::AppResult;
use crate::settings::SETTINGS;
use crate::views::HtmlTemplate;
use askama::Template;
use axum::Extension;
use sqlx::PgPool;

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    pub articles: Vec<Article>,
    pub title: &'static str,
    meili_url: &'static str,
    meili_secret: &'static str,
}

pub async fn articles(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> AppResult<HtmlTemplate<HomePageTemplate>> {
    let user = get_connected_user(user, &db).await?;
    let articles = get_all_active(user.id, &db).await?;
    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Saves",
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}

pub async fn archived(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> AppResult<HtmlTemplate<HomePageTemplate>> {
    let user = get_connected_user(user, &db).await?;
    let articles = get_all_archived(user.id, &db).await?;
    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Archive",
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}

pub async fn starred(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> AppResult<HtmlTemplate<HomePageTemplate>> {
    let user = get_connected_user(user, &db).await?;
    let articles = get_all_starred(user.id, &db).await?;
    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Favorites",
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}

pub async fn tags(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> AppResult<HtmlTemplate<HomePageTemplate>> {
    let user = get_connected_user(user, &db).await?;
    let articles = get_all_active(user.id, &db).await?;
    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Todo",
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}
