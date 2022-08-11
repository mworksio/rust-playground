// use aws-sdk-cloudwatchlogs as cloudwatchlogs;
// use aws_sdk_cloudwatchlogs;
// use cloudwatchlogs;
// use aws_config;
// use tokio;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cloudwatchlogs::{Client, model::InputLogEvent, model::LogStream};
// use std::{time::SystemTime, ops::Sub};
use chrono::Utc;


#[tokio::main]
async fn main() -> Result<(), aws_sdk_cloudwatchlogs::Error> {
    // let config = aws_config::load_from_env().await;
    // let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let region_provider = RegionProviderChain::first_try("us-east-1");
    // let config = aws_config::from_env().region(region_provider).load().await;
    // let client = aws_sdk_cloudwatchlogs::Client::new(&config);

    // ... make some calls with the client
    // let resp = client.put_log_events()
    // let resp = client.put_log_events()
    //     set_log_events(Some(data)).
    //     set_sequence_token(seq).
    //     log_group_name(input).
    //     log_stream_name().
        // build().
        // let resp = client.describe_log_groups().send().await;


        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);

        // let log_events = client
        // .get_log_events()
        // .log_group_name("/aws/events/ebtest")
        // .log_stream_name("123")
        // .send()
        // .await?;
        // println!("{:?}", log_events);

        // let resp = client.describe_log_groups().send().await?;
        // let groups = resp.log_groups().unwrap_or_default();
        // let num_groups = groups.len();
        // println!("{}", num_groups);
        // for group in groups {
        //     println!("  {}", group.log_group_name().unwrap_or_default());
        //     let r = client.describe_log_streams().log_group_name(group.log_group_name().unwrap_or_default()).send().await?;
        //     println!("       {:?}", r);
        // }

        // let input = InputLogEvent{};

        // let lsb = LogStream::builder();
        // let input = lsb.set_log_stream_name(Some("123".to_string())).build();
        // let resp = client.create_log_stream().log_group_name("test").log_stream_name("345").send().await?;
        let streams = client.describe_log_streams().log_group_name("test").send().await?;
        for s in streams.log_streams().unwrap() { 
            if s.log_stream_name().unwrap() == "test" {
                            let next = s.upload_sequence_token().unwrap();
            println!("{:?}", s);
            // let next = s.next().unwrap();
            let builder = InputLogEvent::builder(); 
            let now = Utc::now();
            println!("now: {}", now.timestamp_millis());
            let e = builder.set_message(Some("this is pisa".to_string())).set_timestamp(Some(now.timestamp_millis())).build();
            let resp = client.put_log_events().set_sequence_token(Some(next.to_string())).log_group_name("test").log_stream_name("test").log_events(e).send().await?;
            println!("{:?}", resp);

            }

        }



        // let next = s.upload_sequence_token().unwrap();
        // let next = s.next_token().unwrap();

        // let r = client.describe_log_streams().log_group_name(group.log_group_name().unwrap_or_default()).send().await?;
        // let n = next.to_string();



    Ok(())
}
