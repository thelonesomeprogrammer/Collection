
#[macro_use]

extern crate rocket;

use serde::{Deserialize, Serialize};

use rocket_seek_stream::SeekStream;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;


use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;

use surrealdb::sql::{Thing,Strand,Datetime,Number};


use rocket::{State, serde::{json::Json}};



#[derive(Serialize,Clone,Deserialize)]
struct Featuring {
    out:Vec<Model>
}

#[derive(Serialize,Clone,Deserialize)]
struct made_by {
    out:Vec<Model>
}


#[derive(Serialize,Clone,Deserialize)]
struct Creater{
    id:Option<Thing>,
    description:Option<Strand>,
    network:Option<Strand>,
    url:Option<Strand>,
    logo:Option<Strand>,
    name:Option<Strand>,

}

#[derive(Serialize,Clone,Deserialize)]
struct Model{
    id:Thing,
    name:Strand,
    gender:Option<Strand>,
    birthday:Option<Strand>,
    nationality:Option<Strand>,
    ethnicity:Option<Strand>,
    haircolor:Option<Strand>,
    eye_colour:Option<Strand>,
//    height:Option<Strand>,
//    weight:Option<Strand>,
    fakeboobs:Option<bool>,
    birthplace:Option<Strand>,
    cupsize:Option<Vec<Strand>>,
    bio:Option<Strand>,
    measurements:Option<Strand>,
}

#[derive(Serialize,Clone,Deserialize)]
struct Scene{
    id:Thing,
    filename:Strand,
    title:Strand,
    url:Strand,
    //date:Datetime,
    date:Strand,
    plot:Strand,
    img:Strand,
    //rating:Number,
}

#[derive(Serialize,Clone,Deserialize)]
struct tag{
    id:Thing,
}

#[derive(Serialize,Clone,Deserialize)]
struct Preformere{
    id:Thing,
    name:Strand,
    gender:Strand,
    birthday:Datetime,
    nationality:Strand,
    ethnicity:Strand,
    haircolor:Strand,
    eye_colour:Strand,
    height:Number,
    weight:Number,
    fakeboobs:bool,
    birthplace:Strand,
    cupsize:Vec<Number>,
    bio:Strand,
    measurements:Strand,
}

#[derive(Serialize,Clone,Deserialize)]
struct Studios{
    id:Thing,
    description:Strand,
    network:Strand,
    url:Strand,
    logo:Strand,

}

#[derive(Debug, Deserialize,Serialize,Clone)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

struct RokctDb{
    db:Surreal<surrealdb::engine::remote::ws::Client>,
}
// Return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../ui/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

//Create a route for any url that is a path from the /
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../ui/dist").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}


#[get("/data/<path..>")]
async fn data(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./data/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}


// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

// stream from a given filepath
#[get("/vid/<path..>")]
fn from_path<'a>(path: PathBuf) -> std::io::Result<SeekStream<'a>> {
    let path = PathBuf::from("/home/marrinus/website-data/").join(path);
    SeekStream::from_path(path)
}

#[get("/matadata")]
async fn videos(db:&State<RokctDb>) -> Json<Vec<Scene>> {
    let mut query = db.db.query("SELECT ->made_by.out.*,->featuring.out.*,* FROM scene").await.unwrap();
    let model:Vec<Featuring> = query.take("->featuring").unwrap();
    let stu:Vec<Creater> = query.take("->made_by").unwrap();
    let scenedata:Vec<Scene> = query.take(0).unwrap();
    Json(scenedata.to_vec())
}
#[get("/matadata/<vid_id>")]
async fn video_data(vid_id:String,db:&State<RokctDb>) -> Json<Scene> {
    let query: Scene= db.db.select(("scene",vid_id)).await.unwrap();

    Json(query.clone())
}

#[launch]
async fn rocket() -> _ {
    let db = Surreal::new::<Ws>("127.0.0.1:80").await.expect("no db");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await.expect("sign in failed");
    db.use_ns("test").use_db("test").await.expect("ns");


    rocket::build()
    .manage(RokctDb{db})
    .mount("/", routes![index,static_files,data,from_path,video_data,videos])
    // You must mount the static_files route
}
