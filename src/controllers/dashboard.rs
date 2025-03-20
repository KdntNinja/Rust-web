use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

use crate::models::order::NewOrder;
use crate::services::order::OrderService;
use crate::DbConn;

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

    // Use the order service to handle the business logic
    let _result = conn.run(move |c| OrderService::create_order(&new_order, c));

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

#[get("/dashboard/new-order")]
pub fn new_order_form() -> Template {
    Template::render(
        "pages/dashboard/new_order",
        context! {
            title: "Submit New Order",
        },
    )
}
