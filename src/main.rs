use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};//set header
//when we make request to spotify api were gonna need those three
use serde::{Deserialize, Serialize};//get decserialize, serialize from serde
use std::env;//env for api key

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {//this is to get name of artist for song
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,//like slices in golang, could have multiple artists
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {//track 
    name: String,//will have a name
    href: String,//rhef
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,//list of tracks. 
}

fn print_tracks(tracks: Vec<&Track>) {//function to print our tracks
    //take the vectore of tracks, from api response, pass it here
    for track in tracks {//loop thru tracks
        println!("{}", track.name);//print track name
        println!("{}", track.album.name);//print album anme
        println!(
            "{}",
            track//could have multiple artists
                .album//find album info
                .artists//artist info
                .iter()//iterate
                .map(|artist| artist.name.to_string())//map the artists
                .collect::<String>()//collects strings, print. to above print cmnd
        );
        println!("{}", track.external_urls.spotify);//url for the song
        println!("---------")//seperate for next input
    }
}

#[tokio::main]//this is where we ??>
async fn main() {

    let args: Vec<String> = env::args().collect();//we r passing keyword we wanna search and api key, 
    //pass env with api key, collect those in var called args, a vector of strings

    let search_query = &args[1];
    //first arg is search
    let auth_token = &args[2];
    //api key in second argument. spotify calls it a token
    let url = format!(//format string
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query//pass seach_querry
    );


    let client = reqwest::Client::new();//create clinent to make request
    //using reqwest lib
    let response = client//ur gonna get response back, catch it here

        .get(url)//get request, with url we built with query
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))//make api request, token required for authorization
        .header(CONTENT_TYPE, "application/json")//define
        .header(ACCEPT, "application/json")//define
        .send()//send get request
        .await//
        .unwrap();//needed
    match response.status() {
        reqwest::StatusCode::OK => {//no errors with the request, were gonna do status ok or status not ok
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),//no error, call print tracks
                Err(_) => println!("Hm, the response didn't match the struct/shape we expected."),
                //error handling data
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {//if token is not working, either wrong token, old token 
            println!("Need to grab a new token");
        }
        other => {//any error that isnt cuz of our api token being wrong
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}
