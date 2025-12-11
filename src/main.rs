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
fn App() -> impl IntoView {
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
        <div>
            <h1>"Cash Management Calculator"</h1>

            <div>
                <label>
                    "Monthly Revenue (€): "
                    <input
                        type="number"
                        step="0.01"
                        value={move || monthly_revenue.get()}
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_monthly_revenue.set(value);
                            }
                        }
                    />
                </label>
            </div>

            <div>
                <label>
                    "How often is the cash register counted per month? "
                    <input
                        type="number"
                        value={move || cash_register_count.get()}
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                set_cash_register_count.set(value);
                            }
                        }
                    />
                </label>
            </div>

            <div>
                <label>
                    "What is the average salary of a person counting cash? (€/hour): "
                    <input
                        type="number"
                        step="0.01"
                        value={move || salary_counting.get()}
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_salary_counting.set(value);
                            }
                        }
                    />
                </label>
            </div>

            <div>
                <label>
                    "How often is cash brought to the bank each month? "
                    <input
                        type="number"
                        value={move || bank_trips.get()}
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                set_bank_trips.set(value);
                            }
                        }
                    />
                </label>
            </div>

            <div>
                <label>
                    "What is the salary of the person going to the bank? (€/hour): "
                    <input
                        type="number"
                        step="0.01"
                        value={move || salary_bank_person.get()}
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_salary_bank_person.set(value);
                            }
                        }
                    />
                </label>
            </div>

            <div>
                <label>
                    "Amount of cash transactions per month: "
                    <input
                        type="number"
                        value={move || cash_transactions.get()}
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                set_cash_transactions.set(value);
                            }
                        }
                    />
                </label>
            </div>

            <div>
                <label>
                    "Amount of card transactions per month: "
                    <input
                        type="number"
                        value={move || card_transactions.get()}
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                set_card_transactions.set(value);
                            }
                        }
                    />
                </label>
            </div>

            <h2>"Provider Cost Comparison"</h2>
            <div style="position: relative; display: inline-block;">
                <canvas
                    node_ref=canvas_ref
                    width="900"
                    height="500"
                />
                <button
                    style="position: absolute; top: 10px; right: 10px; padding: 5px 10px; cursor: pointer;"
                    on:click=move |_| set_show_legend.set(!show_legend.get())
                >
                    "Legend"
                </button>

                {move || show_legend.get().then(|| view! {
                    <div style="
                        position: absolute;
                        top: 50px;
                        right: 10px;
                        background: white;
                        border: 2px solid black;
                        padding: 15px;
                        border-radius: 5px;
                        box-shadow: 0 2px 10px rgba(0,0,0,0.2);
                        z-index: 1000;
                    ">
                        <h3 style="margin-top: 0;">"Legend"</h3>
                        <div style="display: flex; align-items: center; margin: 5px 0;">
                            <div style="width: 30px; height: 3px; background: blue; margin-right: 10px;"></div>
                            <span>"SumUp No Commitment (up to 3.5k)"</span>
                        </div>
                        <div style="display: flex; align-items: center; margin: 5px 0;">
                            <div style="width: 30px; height: 3px; background: red; margin-right: 10px;"></div>
                            <span>"SumUp Plus"</span>
                        </div>
                        <div style="display: flex; align-items: center; margin: 5px 0;">
                            <div style="width: 30px; height: 3px; background: green; margin-right: 10px;"></div>
                            <span>"Unzer Go"</span>
                        </div>
                        <div style="display: flex; align-items: center; margin: 5px 0;">
                            <div style="width: 30px; height: 3px; background: black; margin-right: 10px;"></div>
                            <span>"Cash"</span>
                        </div>
                        <button
                            style="margin-top: 10px; width: 100%; padding: 5px;"
                            on:click=move |_| set_show_legend.set(false)
                        >
                            "Close"
                        </button>
                    </div>
                })}
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

    // Calculate x-axis range: 0% to 200% of monthly_revenue in steps of 25%
    let x_max = base_revenue * 2.0;

    // Calculate cash cost (constant, doesn't depend on revenue)
    let cash_cost = cash_register_count as f64 * HOURS_OF_COUNTING * salary_counting
        + bank_trips as f64 * HOURS_OF_DEPOSITING * salary_bank_person
        + SAFEBAG_COSTS * bank_trips as f64;

    // Calculate y-axis range by finding max cost across all providers and cash
    let provider_max = PROVIDERS
        .iter()
        .map(|p| (p.percentage / 100.0) * x_max + p.fixed_fee)
        .fold(0.0f64, f64::max);

    let y_max = provider_max.max(cash_cost) * 1.1; // Add 10% padding

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

    // Draw a line for each provider
    for (i, provider) in PROVIDERS.iter().enumerate() {
        let color = colors[i % colors.len()];

        // Generate points from 0% to 200% in steps of 25%
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

        // Add points for visual clarity
        chart
            .draw_series(
                points
                    .iter()
                    .map(|point| Circle::new(*point, 3, color.filled())),
            )
            .unwrap();
    }

    // Draw cash line (horizontal line since cost is constant)
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

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
