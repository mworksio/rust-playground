use aws_config::meta::region::RegionProviderChain;
use aws_sdk_eventbridge::{Client, model::PutEventRequestEntry};
// use chrono:Utc;


#[tokio::main]
fn main() {
    let region_provider = RegionProviderChain::default_provider().or_
else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    
    let input = PutEventRequestEntry::builder().set_detail(Some("{\"name\": \"pisa\", \"source\": \"local\"}".to_string())).set_event_bus_name("default").set_source("test".to_string()).set_detail_type("test".to_string()).build();
    let resp = client.put_event().set_entries(Some(Vec::<PutEventRequestEntry>(input))).send().await;

    Ok(()) 

}
