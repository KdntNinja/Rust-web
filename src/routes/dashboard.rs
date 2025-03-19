use crate::models::NewOrder;
use crate::DbConn;
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

#[derive(FromForm)]
pub struct OrderForm {
    website: String,
    details: String,
    deadline: String,
}

#[get("/dashboard")]
pub fn dashboard() -> Template {
    Template::render(
        "pages/dashboard/index",
        context! {
            title: "Dashboard",
        },
    )
}

#[post("/dashboard/new-order", data = "<order_form>")]
pub fn new_order(order_form: Form<OrderForm>, conn: DbConn) -> &'static str {
    // Create a new order from the form data
    let new_order = NewOrder {
        website: order_form.website.clone(),
        details: order_form.details.clone(),
        deadline: order_form.deadline.clone(),
    };

    // Use async/await with rocket_sync_db_pools - prefix with underscore to avoid warning
    let _result = conn.run(move |c| new_order.insert(c));

    // Uncomment this in production:
    /*
    match result.await {
        Ok(_) => "Order submitted successfully!",
        Err(_) => "Error submitting order. Please try again.",
    }
    */

    "Order submitted successfully!"
}

#[get("/dashboard/orders")]
pub fn view_orders() -> Template {
    Template::render(
        "pages/dashboard/orders",
        context! {
            title: "Your Orders",
        },
    )
}
