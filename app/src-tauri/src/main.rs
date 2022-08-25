#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use mysql::*;
use mysql::prelude::*;

fn get_movie_list() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Set up DB Connection
    let url = "mysql://root:AcDc16613046334^$@localhost:3306/scotty";
    let pool = Pool::new(url)?;

    // Connect to DB
    let mut conn = pool.get_conn()?;

    // Select movies from the movies table
    let query_str = r"SELECT M.movieID,
	                         M.movieName,
                             M.movieReleaseYear,
                             MR.rating as movieRating,
                             M.movieDesc,
                             M.movieStreamLocation,
                             M.movieCoverLocation
                      FROM scotty.tblMovies M
                      INNER JOIN scotty.tblMovieRatings MR
                      ON MR.ratingID = M.movieRating";
    conn.query_iter(query_str)
        .unwrap()
        .for_each(|row| {
            let r:(i32, Option<String>, i32, Option<String>, Option<String>, Option<String>, Option<String>) = from_row(row.unwrap());
            println!("{}, {}, {}. {}, {}, {}, {}", r.0, r.1.unwrap(), r.2, r.3.unwrap(), r.4.unwrap(), r.5.unwrap(), r.6.unwrap());
        });

    return Ok(());
}

fn main() {
    let result = get_movie_list().unwrap();
    if result == () {

    }
    else {
        println!("AN ERROR HAS OCCURED CONNECTING TO DB");
    }
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

  tauri::Builder::default()
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
