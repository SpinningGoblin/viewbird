use birders::{hotspots::api::NearbyParams, regions::RegionType, Birders, Credentials, Location};

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
        .sub_region_list("CA-BC", RegionType::Subnational2)
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

    let nearby_hotspots = birders
        .nearby_hotspots(
            Location {
                longitude: -123.72295129044818,
                latitude: 48.387562021244186,
            },
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
}
