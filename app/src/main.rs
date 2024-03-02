use birders::{models::regions::RegionType, Birders, Credentials};

#[tokio::main]
async fn main() {
    let api_token = std::env::var("EBIRD_API_KEY").unwrap();
    let credentials = Credentials { api_token };

    let birders = Birders::new(credentials);
    let sub_regions = birders
        .sub_region_list("CA", RegionType::Subnational1)
        .get()
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    let serialized = serde_json::to_string(&sub_regions).unwrap();
    println!("{serialized}");
}
