mod delta_e;
use delta_e::{Coefficients, CIEDE2000};
mod structures;
// use structures::{GetColors};

use lab::Lab;
use axum::{extract::Query, http::Error, routing::get, Json, Router, response::{IntoResponse, Response}};

use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};


#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Colors {
    color_1: Rgb,
    color_2: Rgb,
}

impl Colors {
    pub fn params(&self) -> ([u8; 3], [u8; 3]) {
        (self.color_1.params(), self.color_2.params())
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct Rgb {
    #[serde_as(as = "NoneAsEmptyString")]
    pub r: Option<u8>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub g: Option<u8>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub b: Option<u8>,
}

impl Rgb {
    const DEFAULT_R: u8 = 0;
    const DEFAULT_G: u8 = 0;
    const DEFAULT_B: u8 = 0;

    pub fn params(&self) -> [u8; 3] {
        [
            self.r.unwrap_or(Self::DEFAULT_R),
            self.g.unwrap_or(Self::DEFAULT_G),
            self.b.unwrap_or(Self::DEFAULT_B),
        ]
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self {
            r: Some(Self::DEFAULT_R),
            g: Some(Self::DEFAULT_G),
            b: Some(Self::DEFAULT_B),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            color_1: Rgb::default(),
            color_2: Rgb::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct User {
    name: Fullname,
    age: u8,
}

#[derive(Debug, Deserialize)]
pub struct Fullname {
    first: String,
    last: String,
}

#[tokio::main]
async fn main() {
    let color_1 = Lab { l: 50.0000, a: 2.5000, b: 0.0000 };
    let color_2 = Lab { l: 50.0000, a: 0.0000, b: -2.5000 };
    let coefficients = Coefficients { ksub_l: 1.0, ksub_c: 1.0, ksub_h: 1.0 };
    let delta_e = CIEDE2000::calc_delta_e(&color_1, &color_2, &coefficients);

    let app = Router::new()
        .route("/delta_e", get(calc_delta_e))
        .route("/users", get(hoge))
        .route("/get_colors", get(get_colors));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();


    println!("The color difference is: {}", delta_e);
}

async fn calc_delta_e(query: Option<Query<Colors>>) {
    let (color_1, color_2) = query.unwrap_or_default().params();
    let coefficients = Coefficients { ksub_l: 1.0, ksub_c: 1.0, ksub_h: 1.0 };
    let color1 = Lab::from_rgb(&color_1);
    let color2 = Lab::from_rgb(&color_2);
    let delta_e = CIEDE2000::calc_delta_e(&color1, &color2, &coefficients);
    println!("color_1: {:?}, color_2: {:?}", color_1, color_2);
    println!("The color difference is: {}", delta_e);
}

async fn hoge(user: Query<User>){
    // Json(User {
    //     name: user.name.clone(),
    //     age: user.age,
    // });

    println!("name: {:?}, age: {:?}", user.name.first, user.age);
}

async fn get_colors(query: Option<Query<structures::GetColors>>) -> impl IntoResponse {
    let (color_1, color_2) = query.unwrap_or_default().params();
    let coefficients = Coefficients { ksub_l: 1.0, ksub_c: 1.0, ksub_h: 1.0 };
    let delta_e = CIEDE2000::calc_delta_e_from_rgb(&color_1, &color_2, &coefficients);
    println!("delta_e: {}", delta_e);
    
    Json(structures::Response { delta_e })
}
