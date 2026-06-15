// Peppol Web Frontend — Order Capture, List, and Detail Views
//
// Serves the UBL Order form on :3001.
// Calls peppol-api on :3000 for data.

use axum::extract::Path;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::LazyLock;

static API: LazyLock<String> = LazyLock::new(|| {
    std::env::var("PEPPOL_API_URL").unwrap_or_else(|_| "http://localhost:3000".into())
});

async fn order_form() -> Html<&'static str> {
    Html(include_str!("order_form.html"))
}

// ── Order List ───────────────────────────────────────────────────────

async fn order_list() -> Html<String> {
    let url = format!("{}/api/documents", *API);
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return Html(format!("<p>API error: {e}</p>")),
    };
    let body: Value = resp.json().await.unwrap_or_default();
    let docs = body["documents"].as_array().cloned().unwrap_or_default();

    let rows: String = docs
        .iter()
        .filter(|d| d["document_type"].as_str() == Some("Order"))
        .map(|d| {
            let id = d["document_id"].as_str().unwrap_or("-");
            let uid = d["id"].as_str().unwrap_or("");
            let date = d["created_at"].as_str().unwrap_or("-");
            let valid = d["validated"].as_bool().unwrap_or(false);
            let status = if valid { "✓" } else { "✗" };
            format!(
                "<tr><td><a href=\"/orders/{uid}\">{id}</a></td><td>{status}</td><td>{date}</td></tr>",
            )
        })
        .collect();

    Html(format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>Orders</title>
<style>
body{{font-family:'Fira Code',monospace;background:#0d1117;color:#c9d1d9;padding:20px;max-width:1000px;margin:0 auto}}
h1{{color:#58a6ff}} a{{color:#58a6ff;text-decoration:none}}
table{{width:100%;border-collapse:collapse;margin-top:16px}}
th,td{{padding:10px 14px;text-align:left;border-bottom:1px solid #30363d}}
th{{color:#8b949e;font-size:12px;text-transform:uppercase}}
tr:hover{{background:rgba(255,255,255,.03)}}
.nav{{margin-bottom:20px}} .nav a{{margin-right:16px}}
</style></head><body>
<h1>Orders</h1>
<div class="nav"><a href="/">New Order</a></div>
<table><tr><th>Order ID</th><th>Valid</th><th>Created</th></tr>
{rows}
</table>
</body></html>"#
    ))
}

// ── Order Detail ──────────────────────────────────────────────────────

async fn order_detail(Path(id): Path<String>) -> Html<String> {
    let url = format!("{}/api/documents/{}", *API, id);
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return Html(format!("<p>Error: {e}</p>")),
    };
    if resp.status().as_u16() == 404 {
        return Html("<h1>Order not found</h1>".into());
    }
    let doc: Value = resp.json().await.unwrap_or_default();
    let payload = &doc["payload"];

    Html(render_order_detail(payload))
}

fn render_order_detail(p: &Value) -> String {
    let mut html = String::from(r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>Order Detail</title>
<style>
body{{font-family:'Fira Code',monospace;background:#0d1117;color:#c9d1d9;padding:20px;max-width:900px;margin:0 auto}}
h1{{color:#58a6ff;font-size:22px;margin-bottom:4px}}
h2{{color:#58a6ff;font-size:14px;border-bottom:1px solid #30363d;padding-bottom:4px;margin:24px 0 12px}}
.doc-header{{display:flex;justify-content:space-between;margin-bottom:20px}}
.doc-header .id{{font-size:24px;color:#58a6ff}}
.doc-header .date{{color:#8b949e}}
.section{{margin-bottom:16px}}
.row{{display:flex;justify-content:space-between;padding:4px 0;border-bottom:1px solid #161b22}}
.row .label{{color:#8b949e;font-size:12px}}
.row .value{{font-size:13px;max-width:60%;text-align:right}}
.line{{background:#161b22;border-radius:6px;padding:12px;margin-bottom:8px}}
.line-header{{display:flex;justify-content:space-between;margin-bottom:8px;color:#58a6ff;font-size:13px}}
.amount{{font-weight:bold;color:#3fb950}}
.total-row{{display:flex;justify-content:flex-end;padding:8px 0;font-size:16px}}
.total-row .label{{color:#8b949e;margin-right:16px}}
.total-row .value{{color:#3fb950;font-weight:bold}}
.nav{{margin-bottom:20px}} .nav a{{color:#58a6ff;text-decoration:none;margin-right:16px}}
</style></head><body>
<div class="nav"><a href="/orders">← Orders</a></div>
"#);

    // Header
    let order_id = field(p, &["id", "value"]);
    let date = field(p, &["issue_date"]);
    let buyer = field(p, &["buyer_customer_party", "party", "party_name", "0", "name"]);
    let seller = field(p, &["seller_supplier_party", "party", "party_name", "0", "name"]);
    let currency = field(p, &["document_currency_code", "value"]);

    html.push_str(&format!(
        "<div class=\"doc-header\"><div><div class=\"id\">Order {order_id}</div><div class=\"date\">{date}</div></div><div style=\"text-align:right\"><div>{buyer}</div><div style=\"color:#8b949e\">→</div><div>{seller}</div></div></div>"
    ));

    // Parties
    html.push_str("<h2>Buyer</h2>");
    render_party(&mut html, p, "buyer_customer_party");
    html.push_str("<h2>Seller</h2>");
    render_party(&mut html, p, "seller_supplier_party");

    // Order Lines
    if let Some(lines) = p["order_line"].as_array() {
        html.push_str("<h2>Order Lines</h2>");
        for (i, line) in lines.iter().enumerate() {
            let li = &line["line_item"];
            let line_id = field(li, &["id", "value"]);
            let qty = field(li, &["quantity", "value"]);
            let unit = field(li, &["quantity", "unit_code"]);
            let price = field(li, &["price", "price_amount", "value"]);
            let price_cur = field(li, &["price", "price_amount", "currency_id"]);
            let total = field(li, &["line_extension_amount", "value"]);
            let total_cur = field(li, &["line_extension_amount", "currency_id"]);
            let item_name = field(li, &["item", "name"]);
            let item_desc = field(li, &["item", "description", "value"]);
            let sku = field(li, &["item", "sellers_item_identification", "id", "value"]);

            html.push_str(&format!(
                "<div class=\"line\"><div class=\"line-header\"><span>#{line_id} — {item_name}</span><span class=\"amount\">{currency} {total}</span></div>"
            ));
            html.push_str("<div class=\"row\"><span class=\"label\">Quantity</span><span class=\"value\">{qty} {unit}</span></div>");
            if !price.is_empty() {
                html.push_str(&format!("<div class=\"row\"><span class=\"label\">Unit Price</span><span class=\"value\">{price_cur} {price}</span></div>"));
            }
            if !sku.is_empty() {
                html.push_str(&format!("<div class=\"row\"><span class=\"label\">SKU</span><span class=\"value\">{sku}</span></div>"));
            }
            if !item_desc.is_empty() && item_desc != "null" {
                html.push_str(&format!("<div class=\"row\"><span class=\"label\">Description</span><span class=\"value\">{item_desc}</span></div>"));
            }
            html.push_str("</div>");
        }
    }

    // Totals
    if let Some(mt) = p["anticipated_monetary_total"].as_object() {
        let total = field(&p["anticipated_monetary_total"], &["payable_amount", "value"]);
        if !total.is_empty() {
            html.push_str("<h2>Totals</h2>");
            html.push_str(&format!("<div class=\"total-row\"><span class=\"label\">Total</span><span class=\"value\">{currency} {total}</span></div>"));
        }
    }

    // Delivery
    if let Some(del) = p["delivery"].get(0) {
        let street = field(del, &["delivery_address", "street_name"]);
        let city = field(del, &["delivery_address", "city_name"]);
        let country = field(del, &["delivery_address", "country", "identification_code", "value"]);
        if !street.is_empty() || !city.is_empty() {
            html.push_str("<h2>Delivery</h2>");
            html.push_str(&format!("<div class=\"row\"><span class=\"label\">Address</span><span class=\"value\">{street}, {city}, {country}</span></div>"));
        }
    }

    // Payment
    if let Some(pm) = p["payment_means"].get(0) {
        let code = field(pm, &["payment_means_code", "value"]);
        if !code.is_empty() {
            html.push_str("<h2>Payment</h2>");
            html.push_str(&format!("<div class=\"row\"><span class=\"label\">Means</span><span class=\"value\">{code}</span></div>"));
        }
    }

    html.push_str("</body></html>");
    html
}

fn render_party(html: &mut String, p: &Value, prefix: &str) {
    let party = &p[prefix]["party"];
    let name = field(party, &["party_name", "0", "name"]);
    let street = field(party, &["postal_address", "street_name"]);
    let city = field(party, &["postal_address", "city_name"]);
    let postal = field(party, &["postal_address", "postal_zone", "value"]);
    let country = field(party, &["postal_address", "country", "identification_code", "value"]);
    let vat = field(party, &["party_tax_scheme", "0", "company_id", "value"]);
    let contact = field(party, &["contact", "name"]);
    let email = field(party, &["contact", "electronic_mail", "value"]);
    let phone = field(party, &["contact", "telephone", "value"]);

    if !name.is_empty() {
        html.push_str(&format!("<div class=\"row\"><span class=\"label\">Name</span><span class=\"value\">{name}</span></div>"));
    }
    if !street.is_empty() || !city.is_empty() {
        html.push_str(&format!("<div class=\"row\"><span class=\"label\">Address</span><span class=\"value\">{street}, {city} {postal}, {country}</span></div>"));
    }
    if !vat.is_empty() {
        html.push_str(&format!("<div class=\"row\"><span class=\"label\">VAT</span><span class=\"value\">{vat}</span></div>"));
    }
    if !contact.is_empty() {
        html.push_str(&format!("<div class=\"row\"><span class=\"label\">Contact</span><span class=\"value\">{contact}</span></div>"));
    }
    if !email.is_empty() {
        html.push_str(&format!("<div class=\"row\"><span class=\"label\">Email</span><span class=\"value\">{email}</span></div>"));
    }
}

fn field(v: &Value, path: &[&str]) -> String {
    let mut cur = v;
    for key in path {
        match cur {
            Value::Object(map) => {
                if let Some(val) = map.get(*key) {
                    cur = val;
                } else {
                    return String::new();
                }
            }
            Value::Array(arr) => {
                if let Ok(idx) = key.parse::<usize>() {
                    if let Some(val) = arr.get(idx) {
                        cur = val;
                    } else {
                        return String::new();
                    }
                } else {
                    return String::new();
                }
            }
            _ => return String::new(),
        }
    }
    match cur {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => String::new(),
    }
}

// ── Main ──────────────────────────────────────────────────────────────

fn main() {
    tracing_subscriber::fmt::init();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let app = Router::new()
            .route("/", get(order_form))
            .route("/orders", get(order_list))
            .route("/orders/{id}", get(order_detail));

        let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
        tracing::info!("Peppol Web UI listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}
