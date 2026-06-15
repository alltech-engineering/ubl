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
            let status = if valid {
                "<span class=\"or-status ok\">Valid</span>"
            } else {
                "<span class=\"or-status fail\">Issues</span>"
            };
            format!(
                "<a href=\"/orders/{uid}\" class=\"order-row\"><span class=\"or-id\">{id}</span><span class=\"or-date\">{date}</span>{status}</a>",
            )
        })
        .collect();

    Html(format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Orders</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:#f5f5f5;color:#1a1a1a}
.container{max-width:800px;margin:0 auto;padding:24px 16px}
h1{font-size:22px;margin-bottom:8px}
.subtitle{color:#888;font-size:13px;margin-bottom:20px}
.card{background:#fff;border-radius:12px;overflow:hidden;box-shadow:0 1px 3px rgba(0,0,0,.08)}
.order-row{display:flex;justify-content:space-between;align-items:center;padding:16px 24px;border-bottom:1px solid #f0f0f0;text-decoration:none;color:inherit}
.order-row:last-child{border-bottom:none}
.order-row:hover{background:#fafafa}
.order-row .or-id{font-weight:600;font-size:15px}
.order-row .or-date{font-size:13px;color:#888}
.order-row .or-status{font-size:12px;font-weight:600;padding:4px 10px;border-radius:12px}
.or-status.ok{background:#e6f7e6;color:#1a7a1a}
.or-status.fail{background:#fde8e8;color:#c41e1e}
.nav{margin-bottom:16px}
.nav a{color:#555;text-decoration:none;font-size:14px}
.nav a:hover{color:#000}
.empty{text-align:center;padding:48px;color:#999;font-size:14px}
</style></head><body><div class="container">
<div class="nav"><a href="/">New Order</a></div>
<h1>Orders</h1>
<div class="subtitle">Recently created purchase orders</div>
<div class="card">
{rows}
</div>
</div></body></html>"#
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
    let mut h = String::from(r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Order Detail</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:#f5f5f5;color:#1a1a1a;line-height:1.5}
.container{max-width:800px;margin:0 auto;padding:24px 16px}
.back{color:#555;text-decoration:none;font-size:14px;display:inline-block;margin-bottom:16px}
.back:hover{color:#000}
.card{background:#fff;border-radius:12px;padding:24px;margin-bottom:16px;box-shadow:0 1px 3px rgba(0,0,0,.08)}
.order-header{display:flex;justify-content:space-between;align-items:flex-start;flex-wrap:wrap;gap:12px}
.order-header h1{font-size:22px;font-weight:700}
.order-id{font-family:'SF Mono',monospace;font-size:13px;color:#888;margin-top:2px}
.badge{display:inline-block;padding:4px 12px;border-radius:20px;font-size:12px;font-weight:600;text-transform:uppercase;letter-spacing:.5px}
.badge.ok{background:#e6f7e6;color:#1a7a1a}
.badge.warn{background:#fff8e1;color:#b76e00}
.parties{display:grid;grid-template-columns:1fr 1fr;gap:16px}
@media(max-width:600px){.parties{grid-template-columns:1fr}}
.party-label{font-size:11px;text-transform:uppercase;letter-spacing:1px;color:#999;margin-bottom:8px}
.party-name{font-weight:600;font-size:15px;margin-bottom:4px}
.party-detail{font-size:13px;color:#555;line-height:1.6}
.items-header{display:grid;grid-template-columns:1fr 100px 120px;gap:8px;padding-bottom:8px;border-bottom:2px solid #eee;font-size:11px;text-transform:uppercase;letter-spacing:1px;color:#999}
.item-row{display:grid;grid-template-columns:1fr 100px 120px;gap:8px;padding:12px 0;border-bottom:1px solid #f0f0f0;align-items:center}
.item-name{font-weight:600;font-size:14px}
.item-sku{font-size:12px;color:#888;margin-top:2px}
.item-desc{font-size:12px;color:#777;margin-top:2px}
.item-qty{text-align:center;font-size:14px;color:#555}
.item-total{text-align:right;font-weight:600;font-size:14px}
.totals{margin-top:8px}
.total-line{display:flex;justify-content:flex-end;padding:4px 0;font-size:14px;color:#555;gap:24px}
.total-line.grand{font-size:18px;font-weight:700;color:#000;padding-top:8px;border-top:2px solid #eee;margin-top:8px}
.total-line .tl{text-align:right}
.section-title{font-size:14px;font-weight:600;margin-bottom:12px;color:#333}
.info-grid{display:grid;grid-template-columns:1fr 1fr;gap:8px 24px}
.info-grid .ig-label{font-size:12px;color:#888}
.info-grid .ig-value{font-size:14px}
</style></head><body><div class="container">
<a href="/orders" class="back">← Back to Orders</a>
"#);

    let order_id = field(p, &["id", "value"]);
    let date = field(p, &["issue_date"]);
    let buyer = field(p, &["buyer_customer_party", "party", "party_name", "0", "name"]);
    let seller = field(p, &["seller_supplier_party", "party", "party_name", "0", "name"]);
    let currency = field(p, &["document_currency_code", "value"]);

    // ── Order header card ──
    h.push_str(&format!(r#"
<div class="card">
  <div class="order-header">
    <div>
      <div class="order-id">ORDER</div>
      <h1>{order_id}</h1>
      <div style="font-size:13px;color:#888;margin-top:4px">{date}</div>
    </div>
    <span class="badge ok">Placed</span>
  </div>
</div>"#));

    // ── Parties card ──
    h.push_str(r#"<div class="card"><div class="parties">"#);
    h.push_str("<div><div class=\"party-label\">Sold By</div>");
    h.push_str(&format!("<div class=\"party-name\">{seller}</div><div class=\"party-detail\">"));
    render_party_inline(&mut h, p, "seller_supplier_party");
    h.push_str("</div></div>");

    h.push_str("<div><div class=\"party-label\">Ship To</div>");
    h.push_str(&format!("<div class=\"party-name\">{buyer}</div><div class=\"party-detail\">"));
    render_party_inline(&mut h, p, "buyer_customer_party");
    h.push_str("</div></div>");
    h.push_str("</div></div>");

    // ── Items card ──
    if let Some(lines) = p["order_line"].as_array() {
        h.push_str(&format!(r#"<div class="card">
<div class="section-title">Items ({})</div>
<div class="items-header"><span>Item</span><span>Qty</span><span>Amount</span></div>"#, lines.len()));

        for line in lines {
            let li = &line["line_item"];
            let line_id = field(li, &["id", "value"]);
            let qty = field(li, &["quantity", "value"]);
            let unit = field(li, &["quantity", "unit_code"]);
            let price = field(li, &["price", "price_amount", "value"]);
            let total = field(li, &["line_extension_amount", "value"]);
            let item_name = field(li, &["item", "name"]);
            let sku = field(li, &["item", "sellers_item_identification", "id", "value"]);
            let desc = field(li, &["item", "description", "value"]);

            h.push_str("<div class=\"item-row\">");
            h.push_str(&format!("<div><div class=\"item-name\">{item_name}</div>"));
            if !sku.is_empty() { h.push_str(&format!("<div class=\"item-sku\">SKU: {sku}</div>")); }
            if !desc.is_empty() && desc != "null" { h.push_str(&format!("<div class=\"item-desc\">{desc}</div>")); }
            h.push_str("</div>");
            h.push_str(&format!("<div class=\"item-qty\">{qty} {unit}</div>"));
            h.push_str(&format!("<div class=\"item-total\">{currency} {total}</div>"));
            h.push_str("</div>");
        }

        // Totals
        h.push_str("<div class=\"totals\">");
        let subtotal = field(&p["anticipated_monetary_total"], &["line_extension_amount", "value"]);
        let tax = field(&p["anticipated_monetary_total"], &["tax_inclusive_amount", "value"]);
        let payable = field(&p["anticipated_monetary_total"], &["payable_amount", "value"]);
        if !subtotal.is_empty() {
            h.push_str(&format!("<div class=\"total-line\"><span>Subtotal</span><span class=\"tl\">{currency} {subtotal}</span></div>"));
        }
        if !tax.is_empty() {
            let tax_excl = field(&p["anticipated_monetary_total"], &["tax_exclusive_amount", "value"]);
            if !tax_excl.is_empty() {
                h.push_str(&format!("<div class=\"total-line\"><span>Net</span><span class=\"tl\">{currency} {tax_excl}</span></div>"));
            }
            h.push_str(&format!("<div class=\"total-line\"><span>VAT</span><span class=\"tl\">{currency} {tax}</span></div>"));
        }
        if !payable.is_empty() {
            h.push_str(&format!("<div class=\"total-line grand\"><span>Total</span><span class=\"tl\">{currency} {payable}</span></div>"));
        }
        h.push_str("</div></div>");
    }

    // ── Delivery card (only if has data) ──
    if let Some(del) = p["delivery"].get(0) {
        let street = field(del, &["delivery_address", "street_name"]);
        let city = field(del, &["delivery_address", "city_name"]);
        let postal = field(del, &["delivery_address", "postal_zone", "value"]);
        let country = field(del, &["delivery_address", "country", "identification_code", "value"]);
        if !street.is_empty() || !city.is_empty() {
            h.push_str("<div class=\"card\"><div class=\"section-title\">Delivery Address</div>");
            h.push_str(&format!("<div style=\"font-size:14px;color:#444\">{street}<br>{city} {postal}<br>{country}</div>"));
            h.push_str("</div>");
        }
    }

    // ── Payment card ──
    if let Some(pm) = p["payment_means"].get(0) {
        let code = field(pm, &["payment_means_code", "value"]);
        let terms = field(&p["payment_terms"].get(0).unwrap_or(&Value::Null), &["note", "0", "value"]);
        if !code.is_empty() || !terms.is_empty() {
            h.push_str("<div class=\"card\"><div class=\"section-title\">Payment</div>");
            h.push_str("<div class=\"info-grid\">");
            let code_label = match code.as_str() { "30" => "Credit Transfer", "10" => "Cash", "42" => "Bank Transfer", "48" => "Bank Card", _ => &code };
            if !code.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Method</span><br><span class=\"ig-value\">{code_label}</span></div>")); }
            if !terms.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Terms</span><br><span class=\"ig-value\">{terms}</span></div>")); }
            h.push_str("</div></div>");
        }
    }

    h.push_str("</div></body></html>");
    h
}

fn render_party_inline(h: &mut String, p: &Value, prefix: &str) {
    let party = &p[prefix]["party"];
    let street = field(party, &["postal_address", "street_name"]);
    let city = field(party, &["postal_address", "city_name"]);
    let postal = field(party, &["postal_address", "postal_zone", "value"]);
    let vat = field(party, &["party_tax_scheme", "0", "company_id", "value"]);
    let contact = field(party, &["contact", "name"]);
    let email = field(party, &["contact", "electronic_mail", "value"]);

    if !street.is_empty() { h.push_str(&format!("{street}, {city} {postal}<br>")); }
    if !vat.is_empty() { h.push_str(&format!("VAT: {vat}<br>")); }
    if !contact.is_empty() { h.push_str(&format!("{contact}<br>")); }
    if !email.is_empty() { h.push_str(&format!("{email}")); }
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
