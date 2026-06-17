1|// Peppol Web Frontend — Order Capture, List, and Detail Views
2|//
3|// Serves the UBL Order form on :3001.
4|// Calls peppol-api on :3000 for data.
5|
6|use axum::extract::Path;
7|use axum::response::Html;
8|use axum::routing::get;
9|use axum::Router;
10|use serde_json::Value;
11|use std::net::SocketAddr;
12|use std::sync::LazyLock;
13|
14|static API: LazyLock<String> = LazyLock::new(|| {
15|    std::env::var("PEPPOL_API_URL").unwrap_or_else(|_| "http://localhost:3000".into())
16|});
17|
18|async fn order_form() -> Html<&'static str> {
19|    Html(include_str!("order_form.html"))
20|}
21|
22|// ── Order List ───────────────────────────────────────────────────────
23|
24|// ── CSS Constants ────────────────────────────────────────────────────
25|
26|const CSS_LIST: &str = r##"
27|*{margin:0;padding:0;box-sizing:border-box}
28|body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:#f5f5f5;color:#1a1a1a}
29|.container{max-width:800px;margin:0 auto;padding:24px 16px}
30|h1{font-size:22px;margin-bottom:8px}
31|.subtitle{color:#888;font-size:13px;margin-bottom:20px}
32|.card{background:#fff;border-radius:12px;overflow:hidden;box-shadow:0 1px 3px rgba(0,0,0,.08)}
33|.order-row{display:flex;justify-content:space-between;align-items:center;padding:16px 24px;border-bottom:1px solid #f0f0f0;text-decoration:none;color:inherit}
34|.order-row:last-child{border-bottom:none}
35|.order-row:hover{background:#fafafa}
36|.order-row .or-id{font-weight:600;font-size:15px}
37|.order-row .or-date{font-size:13px;color:#888}
38|.order-row .or-status{font-size:12px;font-weight:600;padding:4px 10px;border-radius:12px}
39|.or-status.ok{background:#e6f7e6;color:#1a7a1a}
40|.or-status.fail{background:#fde8e8;color:#c41e1e}
41|.nav{margin-bottom:16px}
42|.nav a{color:#555;text-decoration:none;font-size:14px}
43|.nav a:hover{color:#000}
44|.empty{text-align:center;padding:48px;color:#999;font-size:14px}
45|"##;
46|
47|const CSS_DETAIL: &str = r##"
48|*{margin:0;padding:0;box-sizing:border-box}
49|body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:#f5f5f5;color:#1a1a1a;line-height:1.5}
50|.container{max-width:800px;margin:0 auto;padding:24px 16px}
51|.back{color:#555;text-decoration:none;font-size:14px;display:inline-block;margin-bottom:16px}
52|.back:hover{color:#000}
53|.card{background:#fff;border-radius:12px;padding:24px;margin-bottom:16px;box-shadow:0 1px 3px rgba(0,0,0,.08)}
54|.order-header{display:flex;justify-content:space-between;align-items:flex-start;flex-wrap:wrap;gap:12px}
55|.order-header h1{font-size:22px;font-weight:700}
56|.order-id{font-family:'SF Mono',monospace;font-size:13px;color:#888;margin-top:2px}
57|.badge{display:inline-block;padding:4px 12px;border-radius:20px;font-size:12px;font-weight:600;text-transform:uppercase;letter-spacing:.5px}
58|.badge.ok{background:#e6f7e6;color:#1a7a1a}
59|.badge.warn{background:#fff8e1;color:#b76e00}
60|.parties{display:grid;grid-template-columns:1fr 1fr;gap:16px}
61|@media(max-width:600px){.parties{grid-template-columns:1fr}}
62|.party-label{font-size:11px;text-transform:uppercase;letter-spacing:1px;color:#999;margin-bottom:8px}
63|.party-name{font-weight:600;font-size:15px;margin-bottom:4px}
64|.party-detail{font-size:13px;color:#555;line-height:1.6}
65|.items-header{display:grid;grid-template-columns:1fr 100px 120px;gap:8px;padding-bottom:8px;border-bottom:2px solid #eee;font-size:11px;text-transform:uppercase;letter-spacing:1px;color:#999}
66|.item-row{display:grid;grid-template-columns:1fr 100px 120px;gap:8px;padding:12px 0;border-bottom:1px solid #f0f0f0;align-items:center}
67|.item-name{font-weight:600;font-size:14px}
68|.item-sku{font-size:12px;color:#888;margin-top:2px}
69|.item-desc{font-size:12px;color:#777;margin-top:2px}
70|.item-qty{text-align:center;font-size:14px;color:#555}
71|.item-total{text-align:right;font-weight:600;font-size:14px}
72|.totals{margin-top:8px}
73|.total-line{display:flex;justify-content:flex-end;padding:4px 0;font-size:14px;color:#555;gap:24px}
74|.total-line.grand{font-size:18px;font-weight:700;color:#000;padding-top:8px;border-top:2px solid #eee;margin-top:8px}
75|.total-line .tl{text-align:right}
76|.section-title{font-size:14px;font-weight:600;margin-bottom:12px;color:#333}
77|.info-grid{display:grid;grid-template-columns:1fr 1fr;gap:8px 24px}
78|.info-grid .ig-label{font-size:12px;color:#888}
79|.info-grid .ig-value{font-size:14px}
.monospace{font-family:"SF Mono",monospace}
80|"##;
81|
82|
83|async fn order_list() -> Html<String> {
84|    let url = format!("{}/api/documents", *API);
85|    let resp = match reqwest::get(&url).await {
86|        Ok(r) => r,
87|        Err(e) => return Html(format!("<p>API error: {e}</p>")),
88|    };
89|    let body: Value = resp.json().await.unwrap_or_default();
90|    let docs = body["documents"].as_array().cloned().unwrap_or_default();
91|
92|    let mut h = String::from(r##"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Orders</title>
93|<style>"##);
94|    h.push_str(CSS_LIST);
95|    h.push_str(r##"</style></head><body><div class="container">
96|<div class="nav"><a href="/">New Order</a></div>
97|<h1>Orders</h1>
98|<div class="subtitle">Recently created purchase orders</div>
99|<div class="card">
100|"##);
101|
102|    let mut rows = String::new();
103|    for d in docs.iter().filter(|d| d["document_type"].as_str() == Some("Order")) {
104|        let id = d["document_id"].as_str().unwrap_or("-");
105|        let uid = d["id"].as_str().unwrap_or("");
106|        let date = d["created_at"].as_str().unwrap_or("-");
107|        let valid = d["validated"].as_bool().unwrap_or(false);
108|        let status = if valid { "<span class=\"or-status ok\">Valid</span>" } else { "<span class=\"or-status fail\">Issues</span>" };
109|        rows.push_str(&format!("<a href=\"/orders/{uid}\" class=\"order-row\"><span class=\"or-id\">{id}</span><span class=\"or-date\">{date}</span>{status}</a>"));
110|    }
111|    if rows.is_empty() {
112|        h.push_str("<div class=\"empty\">No orders yet. <a href=\"/\">Create one</a>.</div>");
113|    } else {
114|        h.push_str(&rows);
115|    }
116|    h.push_str("</div></div></body></html>");
117|    Html(h)
118|}
119|
120|// ── Order Detail ──────────────────────────────────────────────────────
121|
122|async fn order_detail(Path(id): Path<String>) -> Html<String> {
123|    let url = format!("{}/api/documents/{}", *API, id);
124|    let resp = match reqwest::get(&url).await {
125|        Ok(r) => r,
126|        Err(e) => return Html(format!("<p>Error: {e}</p>")),
127|    };
128|    if resp.status().as_u16() == 404 {
129|        return Html("<h1>Order not found</h1>".into());
130|    }
131|    let doc: Value = resp.json().await.unwrap_or_default();
132|    let payload = &doc["payload"];
133|
134|    Html(render_order_detail(payload))
135|}
136|
137|fn render_order_detail(p: &Value) -> String {
    let mut h = String::from(r##"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Order Detail</title>
<style>"## + CSS_DETAIL + r##"</style></head><body><div class="container">
<a href="/orders" class="back">← Back to Orders</a>
"##);

    let order_id = field(p, &["id", "value"]);
    let date = field(p, &["issue_date"]);
    let issue_time = field(p, &["issue_time"]);
    let buyer = field(p, &["buyer_customer_party", "party", "party_name", "0", "name"]);
    let seller = field(p, &["seller_supplier_party", "party", "party_name", "0", "name"]);
    let currency = field(p, &["document_currency_code", "value"]);
    let type_code = field(p, &["order_type_code", "value"]);
    let ref_note = field(p, &["note", "0", "value"]);

    // ── Header card ──
    h.push_str(&format!(r##"<div class="card"><div class="order-header"><div>
<div class="order-id">PURCHASE ORDER</div><h1>{order_id}</h1>
<div style="font-size:13px;color:#888;margin-top:4px">{date}"##));
    if !issue_time.is_empty() { h.push_str(&format!(" at {issue_time}")); }
    h.push_str("</div></div>");
    if !ref_note.is_empty() { h.push_str(&format!("<div style=\"font-size:13px;color:#555;margin-top:8px\">{ref_note}</div>")); }
    h.push_str("<span class=\"badge ok\">Placed</span></div></div>");

    // ── Parties card ──
    h.push_str(r##"<div class="card"><div class="parties">"##);
    // Seller
    h.push_str("<div><div class=\"party-label\">Sold By</div>");
    h.push_str(&format!("<div class=\"party-name\">{seller}</div><div class=\"party-detail\">"));
    render_party_full(&mut h, p, "seller_supplier_party");
    h.push_str("</div></div>");
    // Buyer
    h.push_str("<div><div class=\"party-label\">Bill To / Ship To</div>");
    h.push_str(&format!("<div class=\"party-name\">{buyer}</div><div class=\"party-detail\">"));
    render_party_full(&mut h, p, "buyer_customer_party");
    h.push_str("</div></div>");
    h.push_str("</div></div>");

    // ── Document Metadata ──
    let sales_id = field(p, &["sales_order_id", "value"]);
    let uuid_val = field(p, &["uuid", "value"]);
    let cust_ref = field(p, &["customer_reference", "value"]);
    let cost_code = field(p, &["accounting_cost_code", "value"]);
    let cost = field(p, &["accounting_cost", "value"]);
    let line_cnt = field(p, &["line_count_numeric", "value"]);
    if !sales_id.is_empty() || !uuid_val.is_empty() || !cust_ref.is_empty() || !cost_code.is_empty() {
        h.push_str("<div class=\"card\"><div class=\"section-title\">Details</div><div class=\"info-grid\">");
        if !sales_id.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Sales Order ID</span><br><span class=\"ig-value\">{sales_id}</span></div>")); }
        if !uuid_val.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">UUID</span><br><span class=\"ig-value monospace\">{uuid_val}</span></div>")); }
        if !cust_ref.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Customer Reference</span><br><span class=\"ig-value\">{cust_ref}</span></div>")); }
        if !cost_code.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Cost Code</span><br><span class=\"ig-value\">{cost_code}</span></div>")); }
        if !cost.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Cost</span><br><span class=\"ig-value\">{cost}</span></div>")); }
        if !line_cnt.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Line Count</span><br><span class=\"ig-value\">{line_cnt}</span></div>")); }
        h.push_str("</div></div>");
    }

    // ── Currency ──
    let req_curr = field(p, &["requested_invoice_currency_code", "value"]);
    let pr_curr = field(p, &["pricing_currency_code", "value"]);
    let tx_curr = field(p, &["tax_currency_code", "value"]);
    let quotas = [req_curr, pr_curr, tx_curr];
    if quotas.iter().any(|c| !c.is_empty()) {
        h.push_str("<div class=\"card\"><div class=\"section-title\">Currency</div><div class=\"info-grid\">");
        h.push_str(&format!("<div><span class=\"ig-label\">Document</span><br><span class=\"ig-value\">{currency}</span></div>"));
        if let Some(ref c) = quotas.iter().find(|c| !c.is_empty()) {
            h.push_str(&format!("<div><span class=\"ig-label\">Invoice Currency</span><br><span class=\"ig-value\">{c}</span></div>"));
        }
        h.push_str("</div></div>");
    }

    // ── References ──
    let quot_ref = field(p, &["quotation_document_reference", "id", "value"]);
    let orig_ref = field(p, &["originator_document_reference", "id", "value"]);
    let cat_ref = field(p, &["catalogue_reference", "id", "value"]);
    if !quot_ref.is_empty() || !orig_ref.is_empty() || !cat_ref.is_empty() {
        h.push_str("<div class=\"card\"><div class=\"section-title\">References</div><div class=\"info-grid\">");
        if !quot_ref.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Quotation</span><br><span class=\"ig-value\">{quot_ref}</span></div>")); }
        if !orig_ref.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Originator</span><br><span class=\"ig-value\">{orig_ref}</span></div>")); }
        if !cat_ref.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Catalogue</span><br><span class=\"ig-value\">{cat_ref}</span></div>")); }
        // Order doc references
        if let Some(odrs) = p["order_document_reference"].as_array() {
            for odr in odrs { if let Some(idv) = odr["id"]["value"].as_str() { h.push_str(&format!("<div><span class=\"ig-label\">Order Ref</span><br><span class=\"ig-value\">{idv}</span></div>")); } }
        }
        h.push_str("</div></div>");
    }

    // ── Items ──
    if let Some(lines) = p["order_line"].as_array() {
        h.push_str(&format!(r##"<div class="card"><div class="section-title">Items ({})</div><div class="items-header"><span>Item</span><span>Qty</span><span>Amount</span></div>"##, lines.len()));
        for (idx, line) in lines.iter().enumerate() {
            let li = &line["line_item"];
            let line_id = field(li, &["id", "value"]);
            let qty = field(li, &["quantity", "value"]);
            let unit = field(li, &["quantity", "unit_code"]);
            let total = field(li, &["line_extension_amount", "value"]);
            let item_name = field(li, &["item", "name"]);
            let sku = field(li, &["item", "sellers_item_identification", "id", "value"]);
            let desc = field(li, &["item", "description", "value"]);
            let gtin = field(li, &["item", "standard_item_identification", "id", "value"]);
            let hs = field(li, &["item", "commodity_classification", "0", "item_classification_code", "value"]);
            let price_val = field(li, &["price", "price_amount", "value"]);
            let price_cur = field(li, &["price", "price_amount", "currency_id"]);
            let min_qty = field(li, &["minimum_quantity", "value"]);
            let max_qty = field(li, &["maximum_quantity", "value"]);
            let backorder = field(li, &["back_order_allowed_indicator"]);
            let partial = field(li, &["partial_delivery_indicator"]);
            let status_code = field(li, &["line_status_code", "value"]);
            let tax_amt = field(li, &["total_tax_amount", "value"]);

            h.push_str("<div class=\"item-row\">");
            h.push_str(&format!("<div><div class=\"item-name\">{item_name}</div>"));
            if !sku.is_empty() { h.push_str(&format!("<div class=\"item-sku\">SKU: {sku}</div>")); }
            if !gtin.is_empty() { h.push_str(&format!("<div class=\"item-sku\">GTIN: {gtin}</div>")); }
            if !hs.is_empty() { h.push_str(&format!("<div class=\"item-sku\">HS: {hs}</div>")); }
            if !desc.is_empty() && desc != "null" { h.push_str(&format!("<div class=\"item-desc\">{desc}</div>")); }
            if !price_val.is_empty() { h.push_str(&format!("<div class=\"item-desc\">Unit: {price_cur} {price_val}</div>")); }
            if !min_qty.is_empty() || !max_qty.is_empty() { h.push_str(&format!("<div class=\"item-desc\">Min/Max: {min_qty} / {max_qty}</div>")); }
            if backorder == "true" || partial == "true" { h.push_str(&format!("<div class=\"item-desc\">Backorder: {backorder}, Partial: {partial}</div>")); }
            if !status_code.is_empty() { h.push_str(&format!("<div class=\"item-desc\">Status: {status_code}</div>")); }
            if !tax_amt.is_empty() { h.push_str(&format!("<div class=\"item-desc\">Tax: {currency} {tax_amt}</div>")); }
            // Item properties
            if let Some(props) = li["item"]["item_property"].as_array() {
                for prop in props {
                    let pn = prop["name"]["value"].as_str().unwrap_or("");
                    let pv = prop["value"]["value"].as_str().unwrap_or("");
                    if !pn.is_empty() { h.push_str(&format!("<div class=\"item-desc\">{pn}: {pv}</div>")); }
                }
            }
            h.push_str("</div>");
            h.push_str(&format!("<div class=\"item-qty\">{qty} {unit}</div>"));
            h.push_str(&format!("<div class=\"item-total\">{currency} {total}</div>"));
            h.push_str("</div>");
        }
        // Totals
        h.push_str("<div class=\"totals\">");
        let subtotal = field(&p["anticipated_monetary_total"], &["line_extension_amount", "value"]);
        let net = field(&p["anticipated_monetary_total"], &["tax_exclusive_amount", "value"]);
        let tax = field(&p["anticipated_monetary_total"], &["tax_inclusive_amount", "value"]);
        let payable = field(&p["anticipated_monetary_total"], &["payable_amount", "value"]);
        let allow_total = field(&p["anticipated_monetary_total"], &["allowance_total_amount", "value"]);
        let charge_total = field(&p["anticipated_monetary_total"], &["charge_total_amount", "value"]);
        if !subtotal.is_empty() { h.push_str(&format!("<div class=\"total-line\"><span>Subtotal</span><span class=\"tl\">{currency} {subtotal}</span></div>")); }
        if !allow_total.is_empty() { h.push_str(&format!("<div class=\"total-line\"><span>Allowance</span><span class=\"tl\">{currency} -{allow_total}</span></div>")); }
        if !charge_total.is_empty() { h.push_str(&format!("<div class=\"total-line\"><span>Charge</span><span class=\"tl\">{currency} {charge_total}</span></div>")); }
        if !net.is_empty() { h.push_str(&format!("<div class=\"total-line\"><span>Net</span><span class=\"tl\">{currency} {net}</span></div>")); }
        if !tax.is_empty() { h.push_str(&format!("<div class=\"total-line\"><span>VAT</span><span class=\"tl\">{currency} {tax}</span></div>")); }
        if !payable.is_empty() { h.push_str(&format!("<div class=\"total-line grand\"><span>Total</span><span class=\"tl\">{currency} {payable}</span></div>")); }
        h.push_str("</div></div>");
    }

    // ── Delivery ──
    if let Some(del) = p["delivery"].get(0) {
        let street = field(del, &["delivery_address", "street_name"]);
        let city = field(del, &["delivery_address", "city_name"]);
        let postal = field(del, &["delivery_address", "postal_zone", "value"]);
        let country = field(del, &["delivery_address", "country", "identification_code", "value"]);
        let del_qty = field(del, &["quantity", "value"]);
        let del_unit = field(del, &["quantity", "unit_code"]);
        let req_start = field(del, &["requested_delivery_period", "start_date"]);
        let req_end = field(del, &["requested_delivery_period", "end_date"]);
        let latest = field(del, &["latest_delivery_date"]);
        let terms_note = field(p, &["delivery_terms", "0", "note", "value"]);
        let loss_risk = field(p, &["delivery_terms", "0", "loss_risk", "value"]);
        let has_del = !street.is_empty() || !city.is_empty() || !del_qty.is_empty() || !req_start.is_empty() || !terms_note.is_empty();
        if has_del {
            h.push_str("<div class=\"card\"><div class=\"section-title\">Delivery</div>");
            if !street.is_empty() { h.push_str(&format!("<div style=\"font-size:14px;color:#444;margin-bottom:8px\">{street}<br>{city} {postal}<br>{country}</div>")); }
            h.push_str("<div class=\"info-grid\">");
            if !del_qty.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Quantity</span><br><span class=\"ig-value\">{del_qty} {del_unit}</span></div>")); }
            if !req_start.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Requested</span><br><span class=\"ig-value\">{req_start} – {req_end}</span></div>")); }
            if !latest.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Latest</span><br><span class=\"ig-value\">{latest}</span></div>")); }
            if !terms_note.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Terms</span><br><span class=\"ig-value\">{terms_note}</span></div>")); }
            if !loss_risk.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Loss Risk</span><br><span class=\"ig-value\">{loss_risk}</span></div>")); }
            h.push_str("</div></div>");
        }
    }

    // ── Payment ──
    let pm_code = if let Some(pm) = p["payment_means"].get(0) { field(pm, &["payment_means_code", "value"]) } else { String::new() };
    let pm_due = if let Some(pm) = p["payment_means"].get(0) { field(pm, &["payment_due_date"]) } else { String::new() };
    let terms_note = field(&p["payment_terms"].get(0).unwrap_or(&Value::Null), &["note", "0", "value"]);
    if !pm_code.is_empty() || !terms_note.is_empty() {
        h.push_str("<div class=\"card\"><div class=\"section-title\">Payment</div><div class=\"info-grid\">");
        let code_label = match pm_code.as_str() { "30" => "Credit Transfer", "10" => "Cash", "42" => "Bank Transfer", "48" => "Bank Card", _ => &pm_code };
        if !pm_code.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Method</span><br><span class=\"ig-value\">{code_label}</span></div>")); }
        if !pm_due.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Due Date</span><br><span class=\"ig-value\">{pm_due}</span></div>")); }
        if !terms_note.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Terms</span><br><span class=\"ig-value\">{terms_note}</span></div>")); }
        h.push_str("</div></div>");
    }

    // ── Tax breakdown ──
    if let Some(tt_arr) = p["tax_total"].as_array() {
        for (i, tt) in tt_arr.iter().enumerate() {
            let tax_amt = field(tt, &["tax_amount", "value"]);
            let tax_cur = field(tt, &["tax_amount", "currency_id"]);
            if tax_amt.is_empty() && tax_amt == "0" { continue; }
            h.push_str("<div class=\"card\"><div class=\"section-title\">Tax Breakdown</div><div class=\"info-grid\">");
            h.push_str(&format!("<div><span class=\"ig-label\">Tax Amount</span><br><span class=\"ig-value\">{tax_cur} {tax_amt}</span></div>"));
            if let Some(st) = tt["tax_subtotal"].get(0) {
                let taxable = field(st, &["taxable_amount", "value"]);
                let subtax = field(st, &["tax_amount", "value"]);
                let cat = field(st, &["tax_category", "id", "value"]);
                let scheme = field(st, &["tax_category", "tax_scheme", "id", "value"]);
                if !taxable.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Taxable Amount</span><br><span class=\"ig-value\">{tax_cur} {taxable}</span></div>")); }
                if !subtax.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Tax</span><br><span class=\"ig-value\">{tax_cur} {subtax}</span></div>")); }
                if !cat.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Category</span><br><span class=\"ig-value\">{cat} ({scheme})</span></div>")); }
            }
            h.push_str("</div></div>");
        }
    }

    // ── Allowances & Charges ──
    if let Some(acs) = p["allowance_charge"].as_array() {
        for ac in acs {
            let is_charge = ac["charge_indicator"].as_bool().unwrap_or(false);
            let reason = ac["allowance_charge_reason_code"]["value"].as_str().unwrap_or("");
            let amt = ac["amount"]["value"].as_str().unwrap_or("");
            let amt_cur = ac["amount"]["currency_id"].as_str().unwrap_or("");
            if !amt.is_empty() || !reason.is_empty() {
                let label = if is_charge { "Charge" } else { "Allowance" };
                h.push_str(&format!("<div class=\"card\"><div class=\"section-title\">{label}</div><div class=\"info-grid\">"));
                if !reason.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Reason</span><br><span class=\"ig-value\">{reason}</span></div>")); }
                if !amt.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Amount</span><br><span class=\"ig-value\">{amt_cur} {amt}</span></div>")); }
                h.push_str("</div></div>");
            }
        }
    }

    // ── Exchange Rates ──
    let tax_src = field(p, &["tax_exchange_rate", "source_currency_code", "value"]);
    let tax_rate = field(p, &["tax_exchange_rate", "calculation_rate"]);
    if !tax_src.is_empty() || !tax_rate.is_empty() {
        h.push_str("<div class=\"card\"><div class=\"section-title\">Exchange Rates</div><div class=\"info-grid\">");
        if !tax_src.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Tax Rate</span><br><span class=\"ig-value\">{tax_src} @ {tax_rate}</span></div>")); }
        h.push_str("</div></div>");
    }

    // ── Contract ──
    let contract_id = field(p, &["contract", "0", "id", "value"]);
    let contract_type = field(p, &["contract", "0", "contract_type_code", "value"]);
    if !contract_id.is_empty() {
        h.push_str("<div class=\"card\"><div class=\"section-title\">Contract</div><div class=\"info-grid\">");
        h.push_str(&format!("<div><span class=\"ig-label\">Contract ID</span><br><span class=\"ig-value\">{contract_id}</span></div>"));
        if !contract_type.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Type</span><br><span class=\"ig-value\">{contract_type}</span></div>")); }
        h.push_str("</div></div>");
    }

    // ── Destination Country ──
    let dest = field(p, &["destination_country", "identification_code", "value"]);
    if !dest.is_empty() {
        h.push_str(&format!("<div class=\"card\"><div class=\"section-title\">Destination</div><div style=\"font-size:14px\">{dest}</div></div>"));
    }

    // ── Project References ──
    let proj = field(p, &["project_reference", "0", "id", "value"]);
    if !proj.is_empty() {
        h.push_str(&format!("<div class=\"card\"><div class=\"section-title\">Project</div><div style=\"font-size:14px\">{proj}</div></div>"));
    }

    // ── Additional Parties ──
    let originator = field(p, &["originator_customer_party", "party", "party_name", "0", "name"]);
    let freight = field(p, &["freight_forwarder_party", "party_name", "0", "name"]);
    let acct_buyer = field(p, &["accounting_customer_party", "party", "party_name", "0", "name"]);
    let acct_seller = field(p, &["accounting_supplier_party", "party", "party_name", "0", "name"]);
    if !originator.is_empty() || !freight.is_empty() || !acct_buyer.is_empty() || !acct_seller.is_empty() {
        h.push_str("<div class=\"card\"><div class=\"section-title\">Additional Parties</div><div class=\"info-grid\">");
        if !originator.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Originator</span><br><span class=\"ig-value\">{originator}</span></div>")); }
        if !freight.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Freight Forwarder</span><br><span class=\"ig-value\">{freight}</span></div>")); }
        if !acct_buyer.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Acct. Customer</span><br><span class=\"ig-value\">{acct_buyer}</span></div>")); }
        if !acct_seller.is_empty() { h.push_str(&format!("<div><span class=\"ig-label\">Acct. Supplier</span><br><span class=\"ig-value\">{acct_seller}</span></div>")); }
        h.push_str("</div></div>");
    }

    h.push_str("</div></body></html>");
    h
}

fn render_party_full(h: &mut String, p: &Value, prefix: &str) {
    let party = &p[prefix]["party"];
    let id_val = field(party, &["party_identification", "0", "id", "value"]);
    let id_scheme = field(party, &["party_identification", "0", "id", "scheme_id"]);
    let street = field(party, &["postal_address", "street_name"]);
    let city = field(party, &["postal_address", "city_name"]);
    let postal = field(party, &["postal_address", "postal_zone", "value"]);
    let country = field(party, &["postal_address", "country", "identification_code", "value"]);
    let legal_name = field(party, &["party_legal_entity", "0", "registration_name"]);
    let company_id = field(party, &["party_legal_entity", "0", "company_id", "value"]);
    let vat = field(party, &["party_tax_scheme", "0", "company_id", "value"]);
    let tax_scheme = field(party, &["party_tax_scheme", "0", "tax_scheme", "id", "value"]);
    let endpoint = field(party, &["endpoint_id", "value"]);
    let contact = field(party, &["contact", "name"]);
    let phone = field(party, &["contact", "telephone", "value"]);
    let email = field(party, &["contact", "electronic_mail", "value"]);

    if !street.is_empty() { h.push_str(&format!("{street}<br>")); }
    if !city.is_empty() || !postal.is_empty() { h.push_str(&format!("{city} {postal}<br>")); }
    if !country.is_empty() { h.push_str(&format!("{country}<br>")); }
    if !id_val.is_empty() { h.push_str(&format!("ID: {id_val}")); if !id_scheme.is_empty() { h.push_str(&format!(" ({id_scheme})")); } h.push_str("<br>"); }
    if !legal_name.is_empty() { h.push_str(&format!("Reg: {legal_name}<br>")); }
    if !company_id.is_empty() { h.push_str(&format!("Co: {company_id}<br>")); }
    if !vat.is_empty() { h.push_str(&format!("VAT: {vat}")); if !tax_scheme.is_empty() { h.push_str(&format!(" ({tax_scheme})")); } h.push_str("<br>"); }
    if !endpoint.is_empty() { h.push_str(&format!("Endpoint: {endpoint}<br>")); }
    if !contact.is_empty() { h.push_str(&format!("{contact}<br>")); }
    if !phone.is_empty() { h.push_str(&format!("{phone}<br>")); }
    if !email.is_empty() { h.push_str(&format!("{email}<br>")); }
}

|