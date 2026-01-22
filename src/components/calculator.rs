use leptos::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

struct Provider {
    name: &'static str,
    fixed_fee: f64,
    percentage: f64,
    base_fee_per_transaction: f64,
    colour: &'static str,
}

const PROVIDERS: [Provider; 6] = [
    Provider {
        name: "SumUp No Commitment (up to 3.5k)",
        fixed_fee: 0.0,
        percentage: 1.39,
        base_fee_per_transaction: 0.0,
        colour: "blue",
    },
    Provider {
        name: "SumUp Plus",
        fixed_fee: 19.0,
        percentage: 0.79,
        base_fee_per_transaction: 0.0,
        colour: "red",
    },
    Provider {
        name: "Unzer Go",
        fixed_fee: 0.0,
        percentage: 1.29,
        base_fee_per_transaction: 0.0,
        colour: "green",
    },
    Provider {
        name: "Zettle",
        fixed_fee: 0.0,
        percentage: 1.39,
        base_fee_per_transaction: 0.0,
        colour: "purple",
    },
    Provider {
        name: "Concardis A77",
        fixed_fee: 16.95,
        percentage: 0.99,
        base_fee_per_transaction: 0.0,
        colour: "orange",
    },
    Provider {
        name: "Adyen",
        fixed_fee: 0.00,
        percentage: 0.60,
        base_fee_per_transaction: 0.13,
        colour: "brown",
    },
];

const HOURS_OF_COUNTING: f64 = 0.25;
const HOURS_OF_DEPOSITING: f64 = 0.25;
const SAFEBAG_COSTS: f64 = 10.0;

#[component]
pub fn Calculator() -> impl IntoView {
    let (monthly_revenue, set_monthly_revenue) = signal(10000.0);
    let (cash_register_count, set_cash_register_count) = signal(30);
    let (salary_counting, set_salary_counting) = signal(15.0);
    let (bank_trips, set_bank_trips) = signal(4);
    let (salary_bank_person, set_salary_bank_person) = signal(15.0);
    let (cash_transactions, set_cash_transactions) = signal(200);
    let (card_transactions, set_card_transactions) = signal(500);
    let (show_legend, set_show_legend) = signal(false);
    let (visible_providers, set_visible_providers) = signal(vec![true; PROVIDERS.len()]);
    let (visible_cash, set_visible_cash) = signal(true);

    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        let revenue = monthly_revenue.get();
        let reg_count = cash_register_count.get();
        let sal_counting = salary_counting.get();
        let trips = bank_trips.get();
        let sal_bank = salary_bank_person.get();
        let card_trans = card_transactions.get();
        let visible = visible_providers.get();
        let cash_visible = visible_cash.get();

        if let Some(canvas) = canvas_ref.get() {
            draw_chart(
                &canvas,
                revenue,
                reg_count,
                sal_counting,
                trips,
                sal_bank,
                card_trans,
                &visible,
                cash_visible,
            );
        }
    });

    view! {
        <style>
            "
            .calculator-wrapper {
                max-width: 1200px;
                margin: 0 auto;
                width: 100%;
                padding: 2rem;
            }
            
            .calculator-content {
                display: flex;
                gap: 30px;
                margin-bottom: 40px;
            }
            
            .left-panel {
                flex: 0 0 380px;
                display: flex;
                flex-direction: column;
                gap: 20px;
            }
            
            .right-panel {
                flex: 1;
                min-width: 0;
            }
            
            .input-section {
                background: rgba(255, 255, 255, 0.8);
                border-radius: 16px;
                padding: 24px;
                border: 2px solid rgba(127, 179, 213, 0.2);
                backdrop-filter: blur(10px);
                transition: all 0.3s ease;
                box-shadow: 0 4px 15px rgba(0, 0, 0, 0.08);
            }
            
            .input-section:hover {
                border-color: rgba(127, 179, 213, 0.4);
                box-shadow: 0 8px 25px rgba(0, 0, 0, 0.12);
            }
            
            .input-section h3 {
                color: #7fb3d5;
                font-size: 1.1em;
                font-weight: 600;
                margin-bottom: 18px;
                text-align: center;
                letter-spacing: 0.5px;
            }
            
            .input-group {
                display: flex;
                flex-direction: column;
                gap: 8px;
                margin-bottom: 16px;
            }
            
            .input-group label {
                color: #2d2d2d;
                font-weight: 600;
                font-size: 0.85em;
                text-transform: uppercase;
                letter-spacing: 0.5px;
            }
            
            .input-group input {
                padding: 12px 14px;
                border: 2px solid rgba(127, 179, 213, 0.3);
                border-radius: 10px;
                font-size: 1em;
                transition: all 0.3s ease;
                background: rgba(255, 255, 255, 0.9);
                color: #2d2d2d;
            }
            
            .input-group input:focus {
                outline: none;
                border-color: #7fb3d5;
                box-shadow: 0 0 0 4px rgba(127, 179, 213, 0.15);
                background: white;
            }
            
            .input-group input:hover {
                border-color: #7fb3d5;
            }
            
            .chart-section {
                background: rgba(255, 255, 255, 0.8);
                border-radius: 16px;
                padding: 30px;
                border: 2px solid rgba(127, 179, 213, 0.2);
                backdrop-filter: blur(10px);
                height: 100%;
                box-shadow: 0 4px 15px rgba(0, 0, 0, 0.08);
            }
            
            .chart-section h3 {
                color: #7fb3d5;
                font-size: 1.4em;
                font-weight: 600;
                margin-bottom: 25px;
                text-align: center;
                letter-spacing: 0.5px;
            }
            
            .chart-container {
                position: relative;
                display: flex;
                justify-content: center;
                align-items: center;
            }
            
            canvas {
                border-radius: 12px;
                max-width: 100%;
                height: auto;
            }
            
            .legend-btn {
                position: absolute;
                top: 10px;
                right: 10px;
                padding: 11px 22px;
                background: linear-gradient(135deg, #7fb3d5 0%, #5da0c9 100%);
                color: white;
                border: none;
                border-radius: 10px;
                cursor: pointer;
                font-weight: 600;
                font-size: 0.9em;
                transition: all 0.3s ease;
                box-shadow: 0 4px 12px rgba(127, 179, 213, 0.3);
            }
            
            .legend-btn:hover {
                transform: translateY(-2px);
                box-shadow: 0 6px 16px rgba(127, 179, 213, 0.4);
            }
            
            .legend-popup {
                position: absolute;
                top: 55px;
                right: 10px;
                background: rgba(255, 255, 255, 0.95);
                border: 2px solid rgba(127, 179, 213, 0.3);
                padding: 22px;
                border-radius: 14px;
                box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
                z-index: 1000;
                min-width: 300px;
                backdrop-filter: blur(10px);
            }
            
            .legend-popup h4 {
                margin: 0 0 16px 0;
                color: #7fb3d5;
                font-size: 1.05em;
                font-weight: 600;
            }
            
            .legend-item {
                display: flex;
                align-items: center;
                margin: 12px 0;
                font-size: 0.95em;
                color: #2d2d2d;
                transition: all 0.2s ease;
                padding: 6px 8px;
                border-radius: 8px;
            }
            
            .legend-item:hover {
                background-color: rgba(127, 179, 213, 0.1);
            }
            
            .legend-line {
                width: 40px;
                height: 4px;
                margin-right: 14px;
                border-radius: 2px;
            }
            
            .legend-checkbox {
                width: 18px;
                height: 18px;
                margin-right: 14px;
                cursor: pointer;
                accent-color: #7fb3d5;
                border-radius: 4px;
                transition: all 0.2s ease;
            }
            
            .legend-checkbox:hover {
                transform: scale(1.1);
            }
            
            .close-btn {
                margin-top: 18px;
                width: 100%;
                padding: 12px;
                background: linear-gradient(135deg, #7fb3d5 0%, #5da0c9 100%);
                color: white;
                border: none;
                border-radius: 10px;
                cursor: pointer;
                font-weight: 600;
                transition: all 0.3s ease;
                font-size: 0.95em;
            }
            
            .close-btn:hover {
                transform: translateY(-2px);
                box-shadow: 0 4px 12px rgba(127, 179, 213, 0.3);
            }
            
            .comparison-table-wrapper {
                margin-top: 40px;
                padding: 2rem;
                background: rgba(255, 255, 255, 0.8);
                border-radius: 16px;
                border: 2px solid rgba(127, 179, 213, 0.2);
                backdrop-filter: blur(10px);
                box-shadow: 0 4px 15px rgba(0, 0, 0, 0.08);
            }
            
            .comparison-table-wrapper h2 {
                color: #7fb3d5;
                font-size: 1.4em;
                font-weight: 600;
                margin-bottom: 24px;
                text-align: center;
                letter-spacing: 0.5px;
            }
            
            .table-wrapper {
                overflow-x: auto;
            }
            
            .comparison-table {
                width: 100%;
                border-collapse: collapse;
                font-size: 0.95em;
            }
            
            .comparison-table thead {
                background: linear-gradient(135deg, rgba(127, 179, 213, 0.1) 0%, rgba(224, 244, 255, 0.1) 100%);
                border-bottom: 2px solid rgba(127, 179, 213, 0.3);
            }
            
            .comparison-table th {
                color: #7fb3d5;
                font-weight: 600;
                padding: 16px;
                text-align: left;
                letter-spacing: 0.5px;
            }
            
            .comparison-table td {
                padding: 14px 16px;
                border-bottom: 1px solid rgba(127, 179, 213, 0.1);
                color: #2d2d2d;
            }
            
            .comparison-table tbody tr:hover {
                background: rgba(127, 179, 213, 0.08);
            }
            
            .comparison-table tbody tr:last-child td {
                border-bottom: none;
            }
            
            @media (max-width: 1024px) {
                .calculator-content {
                    flex-direction: column;
                    gap: 20px;
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
                    gap: 15px;
                }
            
                .calculator-wrapper {
                    padding: 1rem;
                }
            
                .input-section {
                    padding: 18px;
                }
            
                .chart-section {
                    padding: 20px;
                }
            
                .left-panel {
                    flex: none;
                    width: 100%;
                }
            }
            "
        </style>

        <div class="calculator-wrapper">
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
                                                {PROVIDERS
                                                    .iter()
                                                    .enumerate()
                                                    .map(|(index, provider)| {
                                                        let visible = visible_providers.get()[index];
                                                        view! {
                                                            <div
                                                                class="legend-item"
                                                                style=move || {
                                                                    format!("opacity: {};", if visible { "1" } else { "0.5" })
                                                                }
                                                            >

                                                                <input
                                                                    type="checkbox"
                                                                    class="legend-checkbox"
                                                                    checked=visible
                                                                    on:change=move |_| {
                                                                        let mut providers = visible_providers.get();
                                                                        providers[index] = !providers[index];
                                                                        set_visible_providers.set(providers);
                                                                    }
                                                                />

                                                                <div
                                                                    class="legend-line"
                                                                    style=format!("background: {};", provider.colour)
                                                                ></div>
                                                                <span>{provider.name}</span>
                                                            </div>
                                                        }
                                                    })
                                                    .collect_view()}
                                                <div
                                                    class="legend-item"
                                                    style=move || {
                                                        format!(
                                                            "opacity: {};",
                                                            if visible_cash.get() { "1" } else { "0.5" },
                                                        )
                                                    }
                                                >

                                                    <input
                                                        type="checkbox"
                                                        class="legend-checkbox"
                                                        checked=move || visible_cash.get()
                                                        on:change=move |_| {
                                                            set_visible_cash.set(!visible_cash.get());
                                                        }
                                                    />

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

            <div class="comparison-table-wrapper">
                <h2>"Provider Comparison"</h2>
                <div class="table-wrapper">
                    <table class="comparison-table">
                        <thead>
                            <tr>
                                <th>"Provider"</th>
                                <th>"Monthly Fee (€)"</th>
                                <th>"Percentage (%)"</th>
                                <th>"Per Transaction Fee (€)"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {PROVIDERS
                                .iter()
                                .map(|provider| {
                                    view! {
                                        <tr>
                                            <td>{provider.name}</td>
                                            <td>{format!("{:.2}", provider.fixed_fee)}</td>
                                            <td>{format!("{:.2}", provider.percentage)}</td>
                                            <td>
                                                {format!("{:.2}", provider.base_fee_per_transaction)}
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </tbody>
                    </table>
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
    card_transactions: i32,
    visible_providers: &[bool],
    cash_visible: bool,
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
        .map(|p| {
            (p.percentage / 100.0) * x_max
                + p.fixed_fee
                + (p.base_fee_per_transaction * card_transactions as f64)
        })
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

    for (i, provider) in PROVIDERS.iter().enumerate() {
        if !visible_providers[i] {
            continue;
        }

        let color = match provider.colour {
            "blue" => &BLUE,
            "red" => &RED,
            "green" => &GREEN,
            "purple" => &MAGENTA,
            "orange" => &YELLOW,
            "brown" => &BLACK,
            _ => &BLUE,
        };

        let points: Vec<(f64, f64)> = (0..=8)
            .map(|step| {
                let x = base_revenue * (step as f64 * 0.25);
                let y = (provider.percentage / 100.0) * x
                    + provider.fixed_fee
                    + (provider.base_fee_per_transaction * card_transactions as f64);
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

    if cash_visible {
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
    }

    root.present().unwrap();
}
