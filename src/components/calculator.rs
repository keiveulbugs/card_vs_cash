use leptos::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

struct Provider {
    name: &'static str,
    fixed_fee: f64,
    percentage: f64,
}

const PROVIDERS: [Provider; 3] = [
    Provider {
        name: "SumUp No Commitment (up to 3.5k)",
        fixed_fee: 0.0,
        percentage: 1.39,
    },
    Provider {
        name: "SumUp Plus",
        fixed_fee: 19.0,
        percentage: 0.79,
    },
    Provider {
        name: "Unzer Go",
        fixed_fee: 0.0,
        percentage: 1.29,
    },
];

const HOURS_OF_COUNTING: f64 = 0.25;
const HOURS_OF_DEPOSITING: f64 = 0.25;
const SAFEBAG_COSTS: f64 = 10.0;

#[component]
pub fn Calculator() -> impl IntoView {
    let (monthly_revenue, set_monthly_revenue) = signal(3000.0);
    let (cash_register_count, set_cash_register_count) = signal(30);
    let (salary_counting, set_salary_counting) = signal(15.0);
    let (bank_trips, set_bank_trips) = signal(4);
    let (salary_bank_person, set_salary_bank_person) = signal(15.0);
    let (cash_transactions, set_cash_transactions) = signal(2000);
    let (card_transactions, set_card_transactions) = signal(5000);
    let (show_legend, set_show_legend) = signal(false);

    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        let revenue = monthly_revenue.get();
        let reg_count = cash_register_count.get();
        let sal_counting = salary_counting.get();
        let trips = bank_trips.get();
        let sal_bank = salary_bank_person.get();

        if let Some(canvas) = canvas_ref.get() {
            draw_chart(&canvas, revenue, reg_count, sal_counting, trips, sal_bank);
        }
    });

    view! {
        <style>
            "
            .calculator-content {
                display: flex;
                gap: 20px;
            }
            
            .left-panel {
                flex: 0 0 400px;
                display: flex;
                flex-direction: column;
                gap: 20px;
            }
            
            .right-panel {
                flex: 1;
                min-width: 0;
            }
            
            .input-section {
                background: #f8f9fa;
                border-radius: 8px;
                padding: 25px;
                border: 1px solid #e0e0e0;
            }
            
            .input-section h3 {
                color: #005a66;
                font-size: 1.2em;
                margin-bottom: 20px;
                text-align: center;
            }
            
            .input-group {
                display: flex;
                flex-direction: column;
                gap: 8px;
                margin-bottom: 18px;
            }
            
            .input-group label {
                color: #005a66;
                font-weight: 600;
                font-size: 0.9em;
            }
            
            .input-group input {
                padding: 10px 14px;
                border: 2px solid #e0e0e0;
                border-radius: 6px;
                font-size: 1em;
                transition: all 0.3s ease;
                background: white;
            }
            
            .input-group input:focus {
                outline: none;
                border-color: #005a66;
                box-shadow: 0 0 0 3px rgba(0, 90, 102, 0.1);
            }
            
            .input-group input:hover {
                border-color: #007a8a;
            }
            
            .chart-section {
                background: white;
                border-radius: 8px;
                padding: 20px;
                border: 1px solid #e0e0e0;
                height: 100%;
            }
            
            .chart-section h3 {
                color: #005a66;
                font-size: 1.5em;
                margin-bottom: 20px;
                text-align: center;
            }
            
            .chart-container {
                position: relative;
                display: flex;
                justify-content: center;
                align-items: center;
            }
            
            canvas {
                border-radius: 8px;
                max-width: 100%;
                height: auto;
            }
            
            .legend-btn {
                position: absolute;
                top: 10px;
                right: 10px;
                padding: 10px 20px;
                background: #005a66;
                color: white;
                border: none;
                border-radius: 6px;
                cursor: pointer;
                font-weight: 600;
                transition: all 0.3s ease;
                box-shadow: 0 2px 8px rgba(0, 90, 102, 0.3);
            }
            
            .legend-btn:hover {
                background: #007a8a;
                transform: translateY(-2px);
                box-shadow: 0 4px 12px rgba(0, 90, 102, 0.4);
            }
            
            .legend-popup {
                position: absolute;
                top: 50px;
                right: 10px;
                background: white;
                border: 2px solid #005a66;
                padding: 20px;
                border-radius: 8px;
                box-shadow: 0 8px 24px rgba(0, 90, 102, 0.3);
                z-index: 1000;
                min-width: 280px;
            }
            
            .legend-popup h4 {
                margin: 0 0 15px 0;
                color: #005a66;
                font-size: 1.1em;
            }
            
            .legend-item {
                display: flex;
                align-items: center;
                margin: 10px 0;
                font-size: 0.95em;
                color: #333;
            }
            
            .legend-line {
                width: 40px;
                height: 3px;
                margin-right: 12px;
                border-radius: 2px;
            }
            
            .close-btn {
                margin-top: 15px;
                width: 100%;
                padding: 10px;
                background: #005a66;
                color: white;
                border: none;
                border-radius: 6px;
                cursor: pointer;
                font-weight: 600;
                transition: all 0.3s ease;
            }
            
            .close-btn:hover {
                background: #007a8a;
            }
            
            @media (max-width: 1024px) {
                .calculator-content {
                    flex-direction: column;
                }
            
                .left-panel {
                    flex: 1;
                    width: 100%;
                }
            
                .right-panel {
                    width: 100%;
                }
            }
            
            @media (max-width: 768px) {
                .calculator-content {
                    flex-direction: column;
                }
            }
            "
        </style>

        <div class="calculator-content">
            <div class="left-panel">
                <div class="input-section">
                    <h3>"Revenue"</h3>
                    <div class="input-group">
                        <label>"Monthly Revenue (€)"</label>
                        <input
                            type="number"
                            step="0.01"
                            value=move || monthly_revenue.get()
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                    set_monthly_revenue.set(value);
                                }
                            }
                        />

                    </div>
                </div>

                <div class="input-section">
                    <h3>"Cash Management"</h3>
                    <div class="input-group">
                        <label>"Cash Register Counted per Month"</label>
                        <input
                            type="number"
                            value=move || cash_register_count.get()
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                    set_cash_register_count.set(value);
                                }
                            }
                        />

                    </div>

                    <div class="input-group">
                        <label>"Salary of Person Counting Cash (€/hour)"</label>
                        <input
                            type="number"
                            step="0.01"
                            value=move || salary_counting.get()
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                    set_salary_counting.set(value);
                                }
                            }
                        />

                    </div>

                    <div class="input-group">
                        <label>"Bank Trips per Month"</label>
                        <input
                            type="number"
                            value=move || bank_trips.get()
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                    set_bank_trips.set(value);
                                }
                            }
                        />

                    </div>

                    <div class="input-group">
                        <label>"Salary of Person Going to Bank (€/hour)"</label>
                        <input
                            type="number"
                            step="0.01"
                            value=move || salary_bank_person.get()
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                    set_salary_bank_person.set(value);
                                }
                            }
                        />

                    </div>
                </div>

                <div class="input-section">
                    <h3>"Transactions"</h3>
                    <div class="input-group">
                        <label>"Cash Transactions per Month"</label>
                        <input
                            type="number"
                            value=move || cash_transactions.get()
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                    set_cash_transactions.set(value);
                                }
                            }
                        />

                    </div>

                    <div class="input-group">
                        <label>"Card Transactions per Month"</label>
                        <input
                            type="number"
                            value=move || card_transactions.get()
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                    set_card_transactions.set(value);
                                }
                            }
                        />

                    </div>
                </div>
            </div>

            <div class="right-panel">
                <div class="chart-section">
                    <h3>"Provider Cost Comparison"</h3>
                    <div class="chart-container">
                        <canvas node_ref=canvas_ref width="900" height="500"></canvas>
                        <button
                            class="legend-btn"
                            on:click=move |_| set_show_legend.set(!show_legend.get())
                        >
                            "Legend"
                        </button>

                        {move || {
                            show_legend
                                .get()
                                .then(|| {
                                    view! {
                                        <div class="legend-popup">
                                            <h4>"Legend"</h4>
                                            <div class="legend-item">
                                                <div class="legend-line" style="background: blue;"></div>
                                                <span>"SumUp No Commitment (up to 3.5k)"</span>
                                            </div>
                                            <div class="legend-item">
                                                <div class="legend-line" style="background: red;"></div>
                                                <span>"SumUp Plus"</span>
                                            </div>
                                            <div class="legend-item">
                                                <div class="legend-line" style="background: green;"></div>
                                                <span>"Unzer Go"</span>
                                            </div>
                                            <div class="legend-item">
                                                <div class="legend-line" style="background: black;"></div>
                                                <span>"Cash"</span>
                                            </div>
                                            <button
                                                class="close-btn"
                                                on:click=move |_| set_show_legend.set(false)
                                            >
                                                "Close"
                                            </button>
                                        </div>
                                    }
                                })
                        }}

                    </div>
                </div>
            </div>
        </div>
    }
}

fn draw_chart(
    canvas: &HtmlCanvasElement,
    base_revenue: f64,
    cash_register_count: i32,
    salary_counting: f64,
    bank_trips: i32,
    salary_bank_person: f64,
) {
    let backend = CanvasBackend::with_canvas_object(canvas.clone()).expect("cannot find canvas");

    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    let x_max = base_revenue * 2.0;

    let cash_cost = cash_register_count as f64 * HOURS_OF_COUNTING * salary_counting
        + bank_trips as f64 * HOURS_OF_DEPOSITING * salary_bank_person
        + SAFEBAG_COSTS * bank_trips as f64;

    let provider_max = PROVIDERS
        .iter()
        .map(|p| (p.percentage / 100.0) * x_max + p.fixed_fee)
        .fold(0.0f64, f64::max);

    let y_max = provider_max.max(cash_cost) * 1.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Provider Costs vs Monthly Revenue", ("sans-serif", 30))
        .margin(15)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..x_max, 0f64..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Monthly Revenue (€)")
        .y_desc("Cost (€)")
        .draw()
        .unwrap();

    let colors = [&BLUE, &RED, &GREEN];

    for (i, provider) in PROVIDERS.iter().enumerate() {
        let color = colors[i % colors.len()];

        let points: Vec<(f64, f64)> = (0..=8)
            .map(|step| {
                let x = base_revenue * (step as f64 * 0.25);
                let y = (provider.percentage / 100.0) * x + provider.fixed_fee;
                (x, y)
            })
            .collect();

        chart
            .draw_series(LineSeries::new(points.clone(), color))
            .unwrap();

        chart
            .draw_series(
                points
                    .iter()
                    .map(|point| Circle::new(*point, 3, color.filled())),
            )
            .unwrap();
    }

    let cash_color = &BLACK;
    let cash_points: Vec<(f64, f64)> = (0..=8)
        .map(|step| {
            let x = base_revenue * (step as f64 * 0.25);
            (x, cash_cost)
        })
        .collect();

    chart
        .draw_series(LineSeries::new(cash_points.clone(), cash_color))
        .unwrap();

    chart
        .draw_series(
            cash_points
                .iter()
                .map(|point| Circle::new(*point, 3, cash_color.filled())),
        )
        .unwrap();

    root.present().unwrap();
}
