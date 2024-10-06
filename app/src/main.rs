use birders::{
    hotspots::api::NearbyParams,
    observations::api::{
        DetailType, NearSpeciesParams, RecentInRegionParams, RecentNearbyNotableParams,
        RecentNearbyParams,
    },
    regions::RegionType,
    Birders, Credentials, Location,
};

#[tokio::main]
async fn main() {
    let api_token = std::env::var("EBIRD_API_KEY").unwrap();
    let credentials = Credentials { api_token };

    println!("starting");

    let birders = if std::env::var("DEBUG_PRINT").is_ok() {
        Birders::new_with_debug_printing(credentials)
    } else {
        Birders::new(credentials)
    };
    let sub_regions = birders
        .sub_region_list("CA-AB", RegionType::Subnational2)
        .get()
        .await
        .unwrap();

    println!("have subregion list");
    let region_info = birders
        .region_info(&sub_regions.get(1).unwrap().code)
        .get()
        .await
        .unwrap();

    println!("have region info");

    let hotspots_in_region = birders
        .hotspots_in_region(&region_info.code, None)
        .get()
        .await
        .unwrap();
    let hotspot = birders
        .hotspot_info(&hotspots_in_region.first().unwrap().loc_id)
        .get()
        .await
        .unwrap();
    let serialized = serde_json::to_string(&hotspot).unwrap();
    println!("{serialized}");

    let location = Location {
        longitude: -113.96707,
        latitude: 50.71641,
    };

    let nearby_hotspots = birders
        .nearby_hotspots(
            &location,
            Some(NearbyParams {
                back: Some(3),
                dist: None,
            }),
        )
        .get()
        .await
        .unwrap();

    let serialized = serde_json::to_string(&nearby_hotspots).unwrap();
    println!("{serialized}");

    birders.adjacent_regions("CA-AB").get().await.unwrap();

    let recent_observations = birders
        .recent_notable_observations_in_region(
            "CA-AB",
            Some(RecentInRegionParams {
                back: Some(10),
                max_results: Some(2),
                r: Some(vec!["CA-AB-SI".to_string()]),
                ..RecentInRegionParams::default()
            }),
        )
        .get()
        .await
        .unwrap();

    let serialized = serde_json::to_string(&recent_observations).unwrap();
    println!("{serialized}");

    let recent_nearby_obs = birders
        .recent_nearby_observations(
            &location,
            Some(RecentNearbyParams {
                max_results: Some(1),
                ..RecentNearbyParams::default()
            }),
        )
        .get()
        .await
        .unwrap();
    let serialized = serde_json::to_string(&recent_nearby_obs).unwrap();
    println!("{serialized}");

    let species_code = "gryjay"; // Just picking out a random one
    let recent_nearby_species_obs = birders
        .recent_nearby_species_observations(species_code, &location, None)
        .get()
        .await
        .unwrap();
    let serialized = serde_json::to_string(&recent_nearby_species_obs).unwrap();
    println!("{serialized}");

    let nearest_of_species = birders
        .nearest_species_observations(
            species_code,
            &location,
            Some(NearSpeciesParams {
                max_results: Some(1),
                ..NearSpeciesParams::default()
            }),
        )
        .get()
        .await
        .unwrap();

    println!("nearest_of_species");

    let serialized = serde_json::to_string(&nearest_of_species).unwrap();
    println!("{serialized}");

    let recent_nearby_notable = birders
        .recent_nearby_notable_observations(
            &location,
            Some(RecentNearbyNotableParams {
                detail: Some(DetailType::Full),
                max_results: Some(1),
                ..RecentNearbyNotableParams::default()
            }),
        )
        .get()
        .await
        .unwrap();

    println!("recent_nearby_notable");

    let serialized = serde_json::to_string(&recent_nearby_notable).unwrap();
    println!("{serialized}");
}
