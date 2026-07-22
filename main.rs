mod category;
mod cipher;
mod config;
mod renderer;
mod repository;
mod sample_data;
mod search;
mod statistics;
mod timeline;

use repository::Repository;
use search::Search;

fn main(){

    let list=

        Repository::load();

    if let Some(cipher)=

        Search::by_name(

            &list,

            config::SEARCH

        ){

        renderer::show(

            &cipher

        );

    }

    timeline::print(

        &list

    );

    statistics::print(

        &list

    );

}
