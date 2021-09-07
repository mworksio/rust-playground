// use html_parser::Dom;
use html_parser::{Dom, Result};
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::fs;
// use serde_json::{Result};

#[derive(Serialize, Deserialize)]
struct UnitTestReport {
  title: String,
  total_tests: String,
  passed: String,
  pass_percentage: String,
  run_duration: String,
  all_results: String,
}

fn main() -> Result<()>  {
    let args: Vec<String> = std::env::args().collect();  
    let path = &args[1];
    let html = fs::read_to_string(path)?; 

    assert!(Dom::parse(&html).is_ok());
    let data = Dom::parse(&html)?.to_json_pretty()?;
    // println!("{}", json);

    let v: Value = serde_json::from_str(&data)?;

    // println!("children = {}", v["children"]);
    // println!("children = {}", v["children"]["children"]);
    // println!("v: {}", v);
    let report = &v["children"][0]["children"][0]["children"];
    // println!("report: {}", report);
    let title = &report[0]["children"][0];
    println!("title: {}", title);

    let total_tests = &report[1]["children"][0]["children"][1]["children"][0];
    println!("total_tests: {}", total_tests);

    let passed = &report[1]["children"][1]["children"][1]["children"][0];
    println!("passed: {}", passed);


    let pass_percentage = &report[1]["children"][2]["children"][1]["children"][0];
    println!("pass_percentage: {}", pass_percentage);

    let run_duration = &report[1]["children"][3]["children"][1]["children"][0];
    println!("run_duration: {}", run_duration);

    let all_results = &report[3]["children"][0];
    println!("all_results: {}", all_results);

    let r = UnitTestReport{ 
        title: title.to_string(),
        total_tests: total_tests.to_string(),
        passed: passed.to_string(),
        pass_percentage: pass_percentage.to_string(), 
        run_duration: run_duration.to_string(),
        all_results: all_results.to_string(),
    };

    let j = serde_json::to_string(&r)?;
    println!("{}", j);

    Ok(())
}
