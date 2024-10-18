use csv::Writer;
use pdf_extract::extract_text_from_mem;
use regex::Regex;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = std::fs::read("./src/files/extrato.pdf")?;

    let text = extract_text_from_mem(&bytes)?;

    let cleaned_text = remove_spaces_in_numbers(&text);

    let mut expenses: Vec<(String, String)> = Vec::new();
    let mut revenues: Vec<(String, String)> = Vec::new();

    let expense_regex = Regex::new(
        r"((?:Compra no debito|Debito automatico|Pix enviado|Pagamento|Aplicacao).*?)\s*-R\$ ?([\d.,]+)",
    )
    .unwrap();
    let revenue_regex =
        Regex::new(r"((?:Pix recebido|Cheque recebido|Credito).*?)\s*R\$ ?([\d.,]+)").unwrap();

    for cap in expense_regex.captures_iter(&cleaned_text) {
        let transaction_name = cap.get(1).map_or("", |m| m.as_str()).to_string();
        let value = cap.get(2).map_or("", |m| m.as_str()).to_string();
        expenses.push((transaction_name, value));
    }

    for cap in revenue_regex.captures_iter(&cleaned_text) {
        let transaction_name = cap.get(1).map_or("", |m| m.as_str()).to_string();
        let value = cap.get(2).map_or("", |m| m.as_str()).to_string();
        revenues.push((transaction_name, value));
    }

    if !expenses.is_empty() {
        let file_name = "expenses.csv";
        let mut wtr = Writer::from_writer(File::create(file_name)?);

        for (name, value) in expenses {
            wtr.write_record(&[value, name])?;
        }
        wtr.flush()?;
        println!("Expenses report generated successfully!");
    }

    if !revenues.is_empty() {
        let file_name = "revenues.csv";
        let mut wtr = Writer::from_writer(File::create(file_name)?);

        for (name, value) in revenues {
            wtr.write_record(&[value, name])?;
        }
        wtr.flush()?;
        println!("Revenues report generated successfully!");
    }

    Ok(())
}

fn remove_spaces_in_numbers(text: &str) -> String {
    let re = Regex::new(r"(?P<number>\d[\d\s.,]*\d)").unwrap();

    re.replace_all(text, |caps: &regex::Captures| {
        let number = caps.name("number").unwrap().as_str();
        number.replace(" ", "")
    })
    .to_string()
}
