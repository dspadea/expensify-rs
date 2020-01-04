use log::Level::Debug;
use reqwest::StatusCode;
use expensify::{Job, ExpensifyConnection, InputSettings, ExpenseCreationJob, Expense};


#[test]
fn create_expense() {

    simple_logger::init_with_level(Debug);

    let api = ExpensifyConnection::new("userid", "secret");
    let expense_creation_job = InputSettings::Expenses( ExpenseCreationJob {
        employee_email: "".to_string(),
        transaction_list: vec![
            Expense {
                merchant: "Dave's House of Pain".to_string(),
                created: "2020-01-01".to_string(),
                amount: 100,
                currency: "USD".to_string(),
                // Optional
                external_id: None,
                category: None,
                tag: None,
                billable: None,
                reimbursable: None,
                comment: None,
                report_id: None,
                policy_id: None,
                tax: None
            }
        ]
    });

    let mut job_desc = Job::from(expense_creation_job);

    let response = api.execute_job(job_desc);

    let status = response.status();

    match status {
        StatusCode::OK => println!("success!"),
        StatusCode::PAYLOAD_TOO_LARGE => {
            println!("Request payload is too large!");
        }
        s => println!("Received response status: {:?}", s),
    };

    println!("Received response: {:?}", response);

    if let Ok(body) = response.text() {
        println!("Body: {}", body);
    }

    assert_eq!(status, StatusCode::OK);

}
