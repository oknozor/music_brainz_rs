use musicbrainz_rs_nova::entity::area::AreaType::*;
use musicbrainz_rs_nova::entity::area::*;
use musicbrainz_rs_nova::Search;

#[tokio::test]
async fn should_search_area() {
    let query = AreaSearchQuery::query_builder()
        .area("London")
        .and()
        .tag("place")
        .build();

    let result = Area::search(query).execute().await.unwrap();

    assert!(result
        .entities
        .iter()
        .any(|area| area.area_type.as_ref().unwrap() == &City));
}
