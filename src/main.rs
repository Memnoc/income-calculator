slint::include_modules!();

const TAXPERCENTAGE: f64 = 0.30;
const OWNERPERCENTAGE: f64 = 0.55;
const PROFITPERCENTAGE: f64 = 0.05;
const OPEXPERCENTAGE: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        // parsing the string number
        let num: f64 = string.trim().parse().unwrap();
        let tax: f64 = num * TAXPERCENTAGE;
        let owner: f64 = num * OWNERPERCENTAGE;
        let profit: f64 = num * PROFITPERCENTAGE;
        let opex: f64 = num * OPEXPERCENTAGE;
        let result = format!(
            "Taxes: {:2}\nOwner: {:2}\nProfit: {:2}\nOpEx: {:2}",
            tax, owner, profit, opex
        );
        ui.set_results(result.into());
    });

    ui.run()
}
