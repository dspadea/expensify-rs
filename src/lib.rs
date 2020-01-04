#[macro_use] extern crate log;

use serde::{ Serialize, Deserialize };

use reqwest::blocking::Response;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExpensifyConnection {
 
    pub api_base: String,
    pub credentials: JobCredentials
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobCredentials {
    #[serde(rename="partnerUserID")]
    pub partner_user_id: String,

    #[serde(rename="partnerUserSecret")]
    pub partner_user_secret: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag="type",
    rename_all = "camelCase"
)]
pub enum InputSettings {
    Expenses(ExpenseCreationJob)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag="type",
    rename_all = "camelCase"
)]
pub enum Job {
    Create(RequestJobDescription)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all = "camelCase"
)]
pub struct RequestJobDescription {
    pub credentials: JobCredentials,
    pub input_settings: InputSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseCreationJob {

    //#[serde(rename="type")]
    //pub job_type: String, // eg "expenses"
    pub employee_email: String,
    pub transaction_list: Vec<Expense>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(
    rename_all = "camelCase"
)]
pub struct Expense {
    pub merchant: String,
    pub created: String, // yyyy-mm-dd
    pub amount: u64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reimbursable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Tax>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tax {
    pub rate_id: String,
    pub amount: Option<u64>
}

impl ExpensifyConnection {
    
    pub fn new(partner_user_id: &str, partner_user_secret: &str ) -> ExpensifyConnection {
        
        ExpensifyConnection{
            api_base: String::from("https://integrations.expensify.com/Integration-Server/ExpensifyIntegrations"),
            credentials: JobCredentials {
                partner_user_id: String::from(partner_user_id),
                partner_user_secret: String::from(partner_user_secret)
            },
        }
        
    }

    pub fn execute_job(&self, job: Job) -> Response {

        let job = match job {
            Job::Create(mut jd) => {
                jd.credentials = self.credentials.clone();
                Job::Create(jd)
            },
        };

        let json = serde_json::to_string_pretty(&job).unwrap();
        println!("job def: {}", json);

        let client = reqwest::blocking::Client::new();

        let mut params = HashMap::new();
        params.insert("requestJobDescription", json);


        let res = client.post(self.api_base.as_str())
            .header("content-type", "application/x-www-form-urlencoded")
            .form(&params)
            .send().unwrap();

        res
    }

}


impl From<InputSettings> for Job {
    fn from(job: InputSettings) -> Self {

        match job {
            InputSettings::Expenses(j) => Job::Create(RequestJobDescription{
                //action_type: "create".to_string(), // TODO - use match to set this
                credentials: JobCredentials {
                    partner_user_id: "".to_string(),
                    partner_user_secret: "".to_string(),
                },
                input_settings: InputSettings::Expenses(j)
            })
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::{ExpensifyConnection, ExpenseCreationJob, Expense, InputSettings, Job};
    use log::Level::Debug;

    #[test]
    fn construct_new() {

        simple_logger::init_with_level(Debug);


        let api = ExpensifyConnection::new("userid", "secret");

        assert_eq!(api.api_base, "https://integrations.expensify.com/Integration-Server/ExpensifyIntegrations");
        assert_eq!(api.credentials.partner_user_id, "userid");
        assert_eq!(api.credentials.partner_user_secret, "secret");
    }

    #[test]
    fn create_expense_job() {

        simple_logger::init_with_level(Debug);

        let expense_creation_job = InputSettings::Expenses( ExpenseCreationJob {
            //job_type: "expenses".to_string(),
            employee_email: "bob@foo.com".to_string(),
            transaction_list: vec![]
        } );

        let job = Job::from(expense_creation_job);

        if let Job::Create(job_desc) = job {
            if let InputSettings::Expenses(j) = job_desc.input_settings {
                assert_eq!(j.employee_email, "bob@foo.com")
            };
        }

    }

}
