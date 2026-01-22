use leptos::prelude::*;
use plotters::prelude::*;
use plotters::style::colors::{self, *};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

struct Provider {
    name: &'static str,
    fixed_fee: f64,
    percentage: f64,
    base_fee_per_transaction: f64,
    colour: plotters::style::RGBColor,
}

const PROVIDERS: [Provider; 7] = [
    Provider {
        name: "SumUp No Commitment",
        fixed_fee: 0.0,
        percentage: 1.39,
        base_fee_per_transaction: 0.0,
        colour: colors::RED,
    },
    Provider {
        name: "SumUp Plus",
        fixed_fee: 19.0,
        percentage: 0.79,
        base_fee_per_transaction: 0.0,
        colour: colors::MAGENTA,
    },
    Provider {
        name: "Unzer Go",
        fixed_fee: 0.0,
        percentage: 1.29,
        base_fee_per_transaction: 0.0,
        colour: colors::GREEN,
    },
    Provider {
        name: "Unzer 5000",
        fixed_fee: 22.90,
        percentage: 0.0,
        base_fee_per_transaction: 0.07,
        colour: colors::CYAN,
    },
    Provider {
        name: "Zettle",
        fixed_fee: 0.0,
        percentage: 1.39,
        base_fee_per_transaction: 0.0,
        colour: colors::MAGENTA,
    },
    Provider {
        name: "Concardis base plan (excl. terminal and yearly costs)",
        fixed_fee: 2.99,
        percentage: 0.065,
        base_fee_per_transaction: 0.02,
        colour: colors::full_palette::ORANGE_900,
    },
    Provider {
        name: "Adyen",
        fixed_fee: 0.00,
        percentage: 0.60,
        base_fee_per_transaction: 0.13,
        colour: colors::full_palette::BLUEGREY_500,
    },
];

#[component]
pub fn Calculator() -> impl IntoView {
    // Standard Inputs
    let (monthly_revenue, set_monthly_revenue) = signal(10000.0);
    let (cash_register_count, set_cash_register_count) = signal(30);
    let (salary_counting, set_salary_counting) = signal(15.0);
    let (bank_trips, set_bank_trips) = signal(4);
    let (salary_bank_person, set_salary_bank_person) = signal(25.0);
    let (card_transactions, set_card_transactions) = signal(500);

    // New Editable Signals (formerly constants)
    let (hours_counting, set_hours_counting) = signal(0.25);
    let (hours_depositing, set_hours_depositing) = signal(0.25);
    let (safebag_costs, set_safebag_costs) = signal(10.0);

    // UI State
    let (visible_providers, set_visible_providers) = signal(vec![true; PROVIDERS.len()]);
    let (visible_cash, set_visible_cash) = signal(true);

    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            draw_chart(
                &canvas,
                monthly_revenue.get(),
                cash_register_count.get(),
                salary_counting.get(),
                bank_trips.get(),
                salary_bank_person.get(),
                card_transactions.get(),
                &visible_providers.get(),
                visible_cash.get(),
                hours_counting.get(),
                hours_depositing.get(),
                safebag_costs.get(),
            );
        }
    });

    view! {
        <style>
            "
            .calculator-wrapper { max-width: 1200px; margin: 0 auto; width: 100%; padding: 2rem; font-family: sans-serif; background: #fcfcfc; }
            .calculator-content { display: flex; gap: 30px; margin-bottom: 40px; }
            .left-panel { flex: 0 0 380px; display: flex; flex-direction: column; gap: 20px; }
            .right-panel { flex: 1; min-width: 0; }
            .input-section { background: white; border-radius: 12px; padding: 20px; border: 1px solid #e0e0e0; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05); }
            .input-section h3 { color: #444; font-size: 1rem; margin-bottom: 15px; border-bottom: 1px solid #eee; padding-bottom: 8px; text-transform: uppercase; letter-spacing: 0.5px;}
            .input-group { display: flex; flex-direction: column; gap: 6px; margin-bottom: 12px; }
            .input-group label { color: #777; font-weight: 600; font-size: 0.7rem; text-transform: uppercase; }
            .input-group input { padding: 8px; border: 1px solid #ccc; border-radius: 6px; font-size: 0.9rem; }
            .chart-section { background: white; border-radius: 12px; padding: 25px; border: 1px solid #e0e0e0; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05); }
            canvas { border-radius: 8px; max-width: 100%; height: auto; }
            .static-legend { width: 100%; display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 10px; padding-top: 20px; border-top: 1px solid #eee; margin-top: 20px; }
            .legend-item { display: flex; align-items: center; padding: 6px 10px; background: #f5f5f5; border-radius: 6px; font-size: 0.8rem; }
            .legend-checkbox { width: 14px; height: 14px; margin-right: 10px; cursor: pointer; }
            .legend-line { width: 20px; height: 3px; margin-right: 10px; border-radius: 1px; }
            .comparison-table-wrapper { margin-top: 40px; padding: 2rem; background: white; border-radius: 12px; border: 1px solid #e0e0e0; }
            .comparison-table { width: 100%; border-collapse: collapse; font-size: 0.9rem; }
            .comparison-table th, .comparison-table td { padding: 12px; text-align: left; border-bottom: 1px solid #eee; }
            .comparison-table th { color: #666; font-weight: 600; }
            "
        </style>

        <div class="calculator-wrapper">
            <div class="calculator-content">
                <div class="left-panel">
                    <div class="input-section">
                        <h3>"General Business"</h3>
                        <div class="input-group">
                            <label>"Monthly Revenue (€)"</label>
                            <input
                                type="number"
                                step="100"
                                value=move || monthly_revenue.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                        set_monthly_revenue.set(v);
                                    }
                                }
                            />

                        </div>
                        <div class="input-group">
                            <label>"Card Transactions / Month"</label>
                            <input
                                type="number"
                                value=move || card_transactions.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<i32>() {
                                        set_card_transactions.set(v);
                                    }
                                }
                            />

                        </div>
                    </div>

                    <div class="input-section">
                        <h3>"Cash Handling Costs"</h3>
                        <div class="input-group">
                            <label>"Monthly Register Counts"</label>
                            <input
                                type="number"
                                value=move || cash_register_count.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<i32>() {
                                        set_cash_register_count.set(v);
                                    }
                                }
                            />

                        </div>
                        <div class="input-group">
                            <label>"Hourly Salary of person handling cash (€)"</label>
                            <input
                                type="number"
                                step="0.5"
                                value=move || salary_counting.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                        set_salary_counting.set(v);
                                    }
                                }
                            />

                        </div>
                        <div class="input-group">
                            <label>"Hourly Salary of person going to the bank (€)"</label>
                            <input
                                type="number"
                                step="0.5"
                                value=move || salary_bank_person.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                        set_salary_bank_person.set(v);
                                    }
                                }
                            />

                        </div>
                        <div class="input-group">
                            <label>"Bank Trips / Month"</label>
                            <input
                                type="number"
                                value=move || bank_trips.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<i32>() {
                                        set_bank_trips.set(v);
                                    }
                                }
                            />

                        </div>
                    </div>

                    <div class="input-section">
                        <h3>"Advanced Cash Settings"</h3>
                        <div class="input-group">
                            <label>"Time per Count (Hours)"</label>
                            <input
                                type="number"
                                step="0.05"
                                value=move || hours_counting.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                        set_hours_counting.set(v);
                                    }
                                }
                            />

                        </div>
                        <div class="input-group">
                            <label>"Time per Bank Trip (Hours)"</label>
                            <input
                                type="number"
                                step="0.05"
                                value=move || hours_depositing.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                        set_hours_depositing.set(v);
                                    }
                                }
                            />

                        </div>
                        <div class="input-group">
                            <label>"Cost per Safebag (€)"</label>
                            <input
                                type="number"
                                step="0.5"
                                value=move || safebag_costs.get()
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                        set_safebag_costs.set(v);
                                    }
                                }
                            />

                        </div>
                    </div>
                </div>

                <div class="right-panel">
                    <div class="chart-section">
                        <canvas node_ref=canvas_ref width="900" height="500"></canvas>

                        <div class="static-legend">
                            {PROVIDERS
                                .iter()
                                .enumerate()
                                .map(|(index, provider)| {
                                    let is_visible = move || visible_providers.get()[index];
                                    view! {
                                        <div
                                            class="legend-item"
                                            style:opacity=move || if is_visible() { "1" } else { "0.5" }
                                        >
                                            <input
                                                type="checkbox"
                                                class="legend-checkbox"
                                                checked=is_visible()
                                                on:change=move |_| {
                                                    let mut v = visible_providers.get();
                                                    v[index] = !v[index];
                                                    set_visible_providers.set(v);
                                                }
                                            />

                                            <div
                                                class="legend-line"
                                                style:background-color=format!(
                                                    "rgb({}, {}, {})",
                                                    provider.colour.0,
                                                    provider.colour.1,
                                                    provider.colour.2,
                                                )
                                            >
                                            </div>
                                            <span>{provider.name}</span>
                                        </div>
                                    }
                                })
                                .collect_view()}
                            <div
                                class="legend-item"
                                style:opacity=move || if visible_cash.get() { "1" } else { "0.5" }
                            >
                                <input
                                    type="checkbox"
                                    class="legend-checkbox"
                                    checked=move || visible_cash.get()
                                    on:change=move |_| set_visible_cash.set(!visible_cash.get())
                                />
                                <div class="legend-line" style="background: black;"></div>
                                <span>"Cash"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="comparison-table-wrapper">
                <h2>"Provider Fee Structure"</h2>
                <table class="comparison-table">
                    <thead>
                        <tr>
                            <th>"Provider"</th>
                            <th>"Monthly Fixed (€)"</th>
                            <th>"Variable Rate (%)"</th>
                            <th>"Per Trans. Fee (€)"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {PROVIDERS
                            .iter()
                            .map(|p| {
                                view! {
                                    <tr>
                                        <td>{p.name}</td>
                                        <td>{format!("{:.2}", p.fixed_fee)}</td>
                                        <td>{format!("{:.2}%", p.percentage)}</td>
                                        <td>{format!("{:.2}", p.base_fee_per_transaction)}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
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
    h_counting: f64,
    h_depositing: f64,
    s_costs: f64,
) {
    let backend = CanvasBackend::with_canvas_object(canvas.clone()).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    let x_max = base_revenue * 2.0;

    // Dynamic Cash Cost calculation based on user signals
    let cash_cost = (cash_register_count as f64 * h_counting * salary_counting)
        + (bank_trips as f64 * h_depositing * salary_bank_person)
        + (s_costs * bank_trips as f64);

    let mut y_max_val = cash_cost;
    for (i, p) in PROVIDERS.iter().enumerate() {
        if visible_providers[i] {
            let p_cost = (p.percentage / 100.0) * x_max
                + p.fixed_fee
                + (p.base_fee_per_transaction * card_transactions as f64);
            if p_cost > y_max_val {
                y_max_val = p_cost;
            }
        }
    }
    let y_max = y_max_val * 1.1;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..x_max, 0f64..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Monthly Revenue (€)")
        .y_desc("Total Cost (€)")
        .draw()
        .unwrap();

    // Draw Provider Lines
    for (i, provider) in PROVIDERS.iter().enumerate() {
        if !visible_providers[i] {
            continue;
        }
        let color = provider.colour;

        let points: Vec<(f64, f64)> = (0..=10)
            .map(|step| {
                let x = (x_max / 10.0) * step as f64;
                let y = (provider.percentage / 100.0) * x
                    + provider.fixed_fee
                    + (provider.base_fee_per_transaction * card_transactions as f64);
                (x, y)
            })
            .collect();

        chart.draw_series(LineSeries::new(points, color)).unwrap();
    }

    // Draw Cash Cost Horizontal Line
    if cash_visible {
        let cash_points = vec![(0.0, cash_cost), (x_max, cash_cost)];
        chart
            .draw_series(LineSeries::new(cash_points, &BLACK.mix(0.8)))
            .unwrap();
    }

    root.present().unwrap();
}
