# Expensify.Com API Client for Rust

## Disclaimer

This is an UNOFFICIAL client for Expensify.com. I am not affiliated with
them in any way. This is for my personal use, and I am making it available
for others to use at their own risk. This code is brand new, extremely
immature, and not ready for production use.

## Usage

This is not published on crates.io at this time. You can use it from Github as follows:

Cargo.toml
```toml
expensify = { git="https://github.com/dspadea/expensify-rs.git" }
```

```rust

    let user = "";
    let secret = "";

    let api = ExpensifyConnection::new(user, secret);
    let expense_creation_job = InputSettings::Expenses( ExpenseCreationJob {
        employee_email: "user@yourco.com".to_string(),
        transaction_list: vec![
            Expense {
                merchant: "Dave's Hobby Shop".to_string(),
                created: "2020-01-03".to_string(),
                amount: 100,
                currency: "USD".to_string(),
                // Optional
                external_id: None,
                category: Some(String::from("Travel")),
                tag: Some(String::from("some_tag")),
                billable: None,
                reimbursable: None,
                comment: Some(String::from("Test expense. Ignore.")),
                report_id: None,
                policy_id: None,
                tax: None
            }
        ]
    });

    let job_desc = Job::from(expense_creation_job);

    let response = api.execute_job(job_desc);

    println!("Received response: {:?}", response);

    match response.status() {
        StatusCode::OK => println!("success!"),
        StatusCode::PAYLOAD_TOO_LARGE => {
            println!("Request payload is too large!");
        }
        s => println!("Received response status: {:?}", s),
    };

    if let Ok(body) = response.text() {
        println!("Body: {}", body);
    }



```