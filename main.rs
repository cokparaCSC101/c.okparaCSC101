use std::fs::DirBuilder;
use std::io::Write;

fn aigbona_juliet(){
    let mut file = std::fs::File::create("Aigbpona_Juliet.txt").expect("create failed");
    let name = "Aigbona Juliet";
    let qualification = "B.Sc.";
    let department = "Consulting";
    let code = "7";
    let consulting = vec!["\nAnalytics consulting services","\nCustomer experience","\nCybersecurity, strategy, risk compliance and resilence", "\nDigital transformation", "\nRisk consulting services", "\nSupply chain and operations", "\nTechnology transformation"];
        for i in 0..consulting.len(){
                file.write_all(consulting[i].as_bytes()).expect("write failed");
                } 
        file.write_all("\n".as_bytes()).expect("write failed");   
        file.write_all("\nName:".as_bytes()).expect("write failed");
        file.write_all(name.as_bytes()).expect("write failed");
        file.write_all("\nQualification:".as_bytes()).expect("write failed");
        file.write_all(qualification.as_bytes()).expect("write failed");
        file.write_all("\nDepartment:".as_bytes()).expect("write failed");
        file.write_all(department.as_bytes()).expect("write failed");
        file.write_all("\nCode:".as_bytes()).expect("write failed");
        file.write_all(code.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");

      
}
fn ehis_ero(){
    let mut file = std::fs::File::create("Ehis_Ero.txt").expect("create failed");
    let name = "Ehis Ero";
    let qualification = "M.Sc.";
    let department = "Strategy";
    let code = "9";
    let strategy = vec!["\nStrategy consulting", "\nCorporate and growth strategy", "\nTransaction strategy and execution", "\nRestructuring and turnaround strategy", "\nIndustry Strategy", "\nDigital business building", "\nCommercial strategy"];
        for i in 0..strategy.len(){
            file.write_all(strategy[i].as_bytes()).expect("write failed");
        }
        file.write_all("\n".as_bytes()).expect("write failed");   
        file.write_all("\nName:".as_bytes()).expect("write failed");
        file.write_all(name.as_bytes()).expect("write failed");
        file.write_all("\nQualification:".as_bytes()).expect("write failed");
        file.write_all(qualification.as_bytes()).expect("write failed");
        file.write_all("\nDepartment:".as_bytes()).expect("write failed");
        file.write_all(department.as_bytes()).expect("write failed");
        file.write_all("\n\nCode:".as_bytes()).expect("write failed");
        file.write_all(code.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
}
fn adamu_sagamu(){
    let mut file = std::fs::File::create("adamu_sagamu.txt").expect("create failed");
    let name = "Adamu Sagamu";
    let qualification = "B.Sc.";
    let department = "Tax";
    let code = "8";
    let tax = vec!["\nTax planning", "\nTax function operations", "\nTax policy and controversy", "\nGlobal trade", "\nTax accounting", "\nTax compliance", "\nTransaction tax"];
        for i in 0..tax.len(){
            file.write_all(tax[i].as_bytes()).expect("write failed");
        }
        file.write_all("\n".as_bytes()).expect("write failed");   
        file.write_all("\nName:".as_bytes()).expect("write failed");
        file.write_all(name.as_bytes()).expect("write failed");
        file.write_all("\nQualification:".as_bytes()).expect("write failed");
        file.write_all(qualification.as_bytes()).expect("write failed");
        file.write_all("\nDepartment:".as_bytes()).expect("write failed");
        file.write_all(department.as_bytes()).expect("write failed");
        file.write_all("\n\nCode:".as_bytes()).expect("write failed");
        file.write_all(code.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
}
fn akpevwe_iloka(){
    let mut file = std::fs::File::create("akpevwe_iloka.txt").expect("create failed");
    let name = "Akpevwe Iloka";
    let qualification = "HND";
    let department = "Assurance";
    let code = "7";
    let assurance = vec!["\nAudit services", "\nClimate change and susutainability services", "\nFinancial accounting advisory services", "\nForensic and integrity services", "\nPrivate client audit experience", "\nAccounting link", "\nAssurance"];
        for i in 0..assurance.len(){
            file.write_all(assurance[i].as_bytes()).expect("write failed");
        }
        file.write_all("\n".as_bytes()).expect("write failed");   
        file.write_all("\nName:".as_bytes()).expect("write failed");
        file.write_all(name.as_bytes()).expect("write failed");
        file.write_all("\nQualification:".as_bytes()).expect("write failed");
        file.write_all(qualification.as_bytes()).expect("write failed");
        file.write_all("\nDepartment:".as_bytes()).expect("write failed");
        file.write_all(department.as_bytes()).expect("write failed");
        file.write_all("\n\nCode:".as_bytes()).expect("write failed");
        file.write_all(code.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
}
fn maria_akinsola(){
    let mut file = std::fs::File::create("maria_akinsola.txt").expect("create failed");
    let name = "Maria Akinsola";
    let qualification = "M.Sc.";
    let department = "Transactions and corporate finance";
    let code = "9";
    let transactions = vec!["\nCorporate Finance", "\nDivestments and carve-outs", "\nSustainability and ESG services", "\nM&A advisory", "\nM&A integration", "\nM&A technology and tools", "\nM&A advanced analytics"];
        for i in 0..transactions.len(){
            file.write_all(transactions[i].as_bytes()).expect("write failed");
        }
        file.write_all("\n".as_bytes()).expect("write failed");   
        file.write_all("\nName:".as_bytes()).expect("write failed");
        file.write_all(name.as_bytes()).expect("write failed");
        file.write_all("\nQualification:".as_bytes()).expect("write failed");
        file.write_all(qualification.as_bytes()).expect("write failed");
        file.write_all("\nDepartment:".as_bytes()).expect("write failed");
        file.write_all(department.as_bytes()).expect("write failed");
        file.write_all("\n\nCode:".as_bytes()).expect("write failed");
        file.write_all(code.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
}
fn gbenga_daniels(){
    let mut file = std::fs::File::create("gbenga_daniels.txt").expect("create failed");
    let name = "Gbenga Daniels";
    let qualification = "HND";
    let department = "People and Workforce";
    let code = "8";
    let paw = vec!["\nChange management and experience", "\nHR transformation", "\nIntegrated workforce mobility", "\nLearning and development consulting", "\nRecognition and reward advisory", "\nWorkforce analytics", "\nPeople and Workforce"];
        for i in 0..paw.len(){
            file.write_all(paw[i].as_bytes()).expect("write failed");
        }
        file.write_all("\n".as_bytes()).expect("write failed");   
        file.write_all("\nName:".as_bytes()).expect("write failed");
        file.write_all(name.as_bytes()).expect("write failed");
        file.write_all("\nQualification:".as_bytes()).expect("write failed");
        file.write_all(qualification.as_bytes()).expect("write failed");
        file.write_all("\nDepartment:".as_bytes()).expect("write failed");
        file.write_all(department.as_bytes()).expect("write failed");
        file.write_all("\n\nCode:".as_bytes()).expect("write failed");
        file.write_all(code.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
}
fn main(){
    let mut name = "";
    let mut qualification = "";
    let mut department = "";
    let mut code = "";

    let functions = vec![aigbona_juliet(), ehis_ero(), adamu_sagamu(), akpevwe_iloka(), maria_akinsola(), gbenga_daniels()];

    for i in functions {
        println!("\nData written to file.");
    }
}